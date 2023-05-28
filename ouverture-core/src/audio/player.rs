use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::TimeBase;

use crate::music::song::*;
use std::fs::File;

use log::{debug, info, warn};

use super::output;

use std::time;

use tokio::sync::broadcast::{ Sender, Receiver};
use rc_event_queue::mpmc::{DefaultSettings, EventQueue, EventReader};
use rc_event_queue::prelude::*;

use std::thread;

use std::pin::Pin;
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub enum AudioEvent {
    Finished,
    Failed,
}

// The audio thread responds immediately (or at least ASAP)
// to these commands
#[derive(Debug, Clone)]
pub enum AudioCommand {
    PlayNew(Song),
    Play,
    Pause,
    Seek(f64),
    Done, // signal the current song has finished playing
    Quit, // quit loops and get ready to exit this thread
}
use AudioCommand::*;

pub struct AudioThread {
    handle: std::thread::JoinHandle<()>,
    pub current: Option<Song>,
}

impl std::fmt::Debug for AudioThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioThread")
            .field("handle", &self.handle)
            .field("event_queue", &"<event_queue>")
            .field("current", &self.current)
            .finish()
    }
}

pub fn start_audio_thread(rx: Receiver<AudioCommand>, tx: Sender<AudioEvent>) -> AudioThread {

    let handle = std::thread::spawn(|| audio_thread_fn(rx,tx));
    debug!("audio thread started");

    AudioThread {
        handle,
        current: None,
    }
}
pub fn audio_thread_send_cmd(cmd: AudioCommand, tx: &Sender<AudioCommand>) {
    tx.send(cmd).unwrap();
    debug!("audio thread cmd passed");
}

pub fn stop_audio_thread(audio_thread: AudioThread) {
    let handle = audio_thread.handle;
    handle.join().unwrap();
}

const AUDIO_THREAD_EQ_POLL_PERIOD_MS: u64 = 50;

fn audio_thread_fn(
    mut rx: Receiver<AudioCommand>,
    tx: Sender<AudioEvent>,
) {
    let mut current_seek = 0;
    let mut current_song = None;
    let mut command_from_decode_loop: Option<AudioCommand> = None;
    loop {
        let do_cmd = if let Some(cmd) = command_from_decode_loop.clone() {
            debug!("audio thread interrupted by command: {:?}", cmd);
            Some(cmd)
        } else {
            if let Ok(cmd) = rx.try_recv() {
                Some(cmd.clone())
            } else {
                None
            }
        };
        command_from_decode_loop = match do_cmd {
            None => {
                let sleep_period = time::Duration::from_millis(AUDIO_THREAD_EQ_POLL_PERIOD_MS);
                thread::sleep(sleep_period);
                None
            }
            Some(Quit) => break,
            Some(PlayNew(song)) => {
                current_song = Some(song.clone());
                current_seek = 0;
                decode(&song, &mut rx, &mut current_seek)
            }
            Some(Play) => {
                if let Some(song) = &current_song {
                    debug!(
                        "resuming play of current song: {:?} at seek {current_seek}",
                        song
                    );
                    decode(&song, &mut rx, &mut current_seek)
                } else {
                    warn!("Play requested but no current song");
                    None
                }
            }
            Some(Pause) => None,
            _ => unimplemented!(),
        }
    }
}

pub fn decode(
    song: &Song,
    rx: &mut Receiver<AudioCommand>,
    seek: &mut u64,
) -> Option<AudioCommand> {
    if let Some(SongSource::FilePath(song_source_filepath)) = song.clone().source {
        let song_src = File::open(&song_source_filepath).unwrap();
        info!("trying to play file {:?}", song_src);

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(song_src), Default::default());

        let mut hint = Hint::new();
        hint.with_extension(&song.format.to_string());

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let mut fmt_opts: FormatOptions = Default::default();
        fmt_opts.enable_gapless = true;

        // Probe the media source.
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("unsupported format");

        // Get the instantiated format reader.
        let mut format = probed.format; // TODO ? check formats match

        // Find the first audio track with a known (decodeable) codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("no supported audio tracks");
        // Use the default options for the decoder.
        let dec_opts: DecoderOptions = Default::default();

        // Create a decoder for the track.
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");

        // Store the track identifier, it will be used to filter packets.

        let track_id = track.id;

        let mut audio_output = None;

        let no_progress = false;

        // Get the selected track's timebase and duration.
        let tb = track.codec_params.time_base;
        let dur = track
            .codec_params
            .n_frames
            .map(|frames| track.codec_params.start_ts + frames);

        // The decode loop.
        let result: Result<(), Error> = loop {
            // Get the next packet from the media format.
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::ResetRequired) => {
                    // The track list has been changed. Re-examine it and create a new set of decoders,
                    // then restart the decode loop. This is an advanced feature and it is not
                    // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                    // for chained OGG physical streams.
                    unimplemented!();
                }
                Err(err) => {
                    // A unrecoverable error occured, halt decoding.
                    break Err(err);
                }
            };

            // Consume any new metadata that has been read since the last packet.
            while !format.metadata().is_latest() {
                // Pop the old head of the metadata queue.
                format.metadata().pop();

                // Consume the new metadata at the head of the metadata queue.
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    // If the audio output is not open, try to open it.
                    if audio_output.is_none() {
                        // Get the audio buffer specification. This is a description of the decoded
                        // audio buffer's sample format and sample rate.
                        let spec = *decoded.spec();

                        // Get the capacity of the decoded buffer. Note that this is capacity, not
                        // length! The capacity of the decoded buffer is constant for the life of the
                        // decoder, but the length is not.
                        let duration = decoded.capacity() as u64;

                        // Try to open the audio output.
                        audio_output.replace(output::try_open(spec, duration).unwrap());
                    } else {
                        // TODO: Check the audio spec. and duration hasn't changed.
                    }
                    // Write the decoded audio samples to the audio output if the presentation timestamp
                    // for the packet is >= the seeked position (0 if not seeking).
                    if packet.ts() >= *seek {
                        if !no_progress {
                            print_progress(packet.ts(), dur, tb);
                        }

                        if let Some(ref mut audio_output) = audio_output {
                            audio_output.write(decoded).unwrap()
                        }

                        // update seek time
                        *seek = packet.ts();
                    }

                    // check for any command
                    match rx.try_recv() {
                        Ok(Play) => (),
                        Ok(cmd) => return Some(cmd.clone()), // TODO exhaust the enum manually to avoid alloc in audio thread
                        Err(_) => (), // most likely no new cmd
                     }
                }
                Err(Error::IoError(_)) => {
                    // The packet failed to decode due to an IO error, skip the packet.
                    continue;
                }
                Err(Error::DecodeError(_)) => {
                    // The packet failed to decode due to invalid data, skip the packet.
                    continue;
                }
                Err(err) => {
                    // An unrecoverable error occured, halt decoding.
                    break Err(err);
                }
            }
        }; // EOL
        info!("result playing track: {:?}", result);
        *seek = 0; // reset seek time
    }
    info!("song does not have local filepath ({:?})", song);

    return None;
}

fn print_progress(ts: u64, dur: Option<u64>, tb: Option<TimeBase>) {
    //debug!("progressing");
}
