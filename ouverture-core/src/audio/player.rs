use symphonia::core::audio::AudioBuffer;
use symphonia::core::codecs::CodecParameters;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::{Time, TimeBase};

use crate::music::song::*;
use std::fs::File;

use log::{debug, info};

use super::output;

use std::thread::{spawn, JoinHandle};

use rc_event_queue::mpmc::{EventQueue, EventReader};
use rc_event_queue::prelude::*;
use std::sync::Arc;
use std::pin::Pin;

// The audio thread responds immediately (or at least ASAP)
// to these commands
#[derive(Debug, Clone)]
pub enum AudioCommand {
    Play,
    Pause,
    Toggle,
    Restart,
    Seek(f64),
    New(Song),
    Quit, // quit loops and get ready to exit this thread
}
pub struct AudioThread {
    handle: JoinHandle<()>,
    event_queue: Pin<Arc<EventQueue<AudioCommand>>>,
}

#[derive(Copy, Clone)]
struct PlayTrackOptions {
    track_id: u32,
    seek_ts: u64,
}

pub fn start_audio_thread() -> AudioThread {
    let handle = spawn(audio_thread);
    let event_queue = EventQueue::<AudioCommand>::new();
    return AudioThread {
        handle,
        event_queue,
    };
}
fn audio_thread() {

    loop {


    }

}

pub fn stop_audio_thread(audio_thread: AudioThread) {
    return audio_thread.handle.join().unwrap();
}

pub fn play(song: Song) {
    if let Some(SongSource::FilePath(song_source_filepath)) = song.source {
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

        let seek_ts = 0;
        let track_id = track.id;
        let mut play_opts = PlayTrackOptions { track_id, seek_ts };

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
            if packet.track_id() != play_opts.track_id {
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
                    if packet.ts() >= play_opts.seek_ts {
                        if !no_progress {
                            print_progress(packet.ts(), dur, tb);
                        }

                        if let Some(ref mut audio_output) = audio_output {
                            audio_output.write(decoded).unwrap()
                        }
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
    }
}

fn print_progress(ts: u64, dur: Option<u64>, tb: Option<TimeBase>) {
    //debug!("progressing");
}
