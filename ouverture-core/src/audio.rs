mod output;
mod player;

use tokio::task::spawn;

use log::{debug, trace, warn};
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::music::song::Song;

use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::{self, Duration, Instant};

pub use player::{start_audio_thread, stop_audio_thread, AudioThread};

pub use player::{audio_thread_send_cmd, AudioCommand, AudioEvent};

pub fn audio_thread_play_song(tx: &Sender<AudioCommand>, song: Song) {
    audio_thread_send_cmd(AudioCommand::PlayNew(song), tx);
}

pub fn audio_thread_play(tx: &Sender<AudioCommand>) {
    audio_thread_send_cmd(AudioCommand::Play, tx);
}

pub fn audio_thread_pause(tx: &Sender<AudioCommand>) {
    audio_thread_send_cmd(AudioCommand::Pause, tx);
}
pub fn audio_thread_stop(tx: &Sender<AudioCommand>) {
    audio_thread_send_cmd(AudioCommand::Stop, tx);
}

#[derive(Clone)]
pub struct AudioState {
    pub cmd_tx: Sender<AudioCommand>,
    pub current_song: Option<Song>,
    pub current_seek: f32,
    paused: bool,
    events_received: u64, //count the number of event received
    queue_future: VecDeque<Song>,
    queue_past: VecDeque<Song>,
}

const AUDIO_GET_SEEK_POLL_FREQ_MS: u64 = 20;

impl AudioState {
    pub fn play(&mut self, opt_song: Option<Song>) {
        if let Some(song) = opt_song {
            audio_thread_play_song(&self.cmd_tx, song.clone());
            self.current_song = Some(song);
            self.paused = false;
        } else {
            if self.current_song.is_some() {
                audio_thread_play(&self.cmd_tx);
                self.paused = false;
            }
        }
    }
    pub fn pause(&mut self) {
        audio_thread_pause(&self.cmd_tx);
        self.paused = true;
    }
    pub fn toggle(&mut self) {
        if self.paused {
            self.play(None);
        } else {
            self.pause();
        }
    }

    pub fn enqueue(&mut self, song: Song) {
        self.queue_future.push_back(song);
    }

    pub fn next(&mut self) {
        let opt_song = self.queue_future.pop_front();
        if opt_song.is_some() {
            if let Some(song) = &self.current_song {
                self.queue_past.push_back(song.clone());
            }
            self.play(opt_song);
        } else {
            if let Some(song) = &self.current_song {
                self.queue_past.push_back(song.clone());
            }
            audio_thread_stop(&self.cmd_tx);
            self.current_song = None;
            self.paused = true;
        }
    }
    pub fn previous(&mut self) {
        // TODO resume from start if seek < 5s
        let opt_song = self.queue_past.pop_back();
        if opt_song.is_some() {
            if let Some(current_song) = &self.current_song {
                self.queue_future.push_front(current_song.clone());
            }
            self.play(opt_song);
        }
    }

    pub async fn get_seek(audio_state: Arc<Mutex<AudioState>>) -> f32 {
        let event_count_before = audio_state.lock().unwrap().events_received;
        audio_thread_send_cmd(AudioCommand::GetSeek, &audio_state.lock().unwrap().cmd_tx);
        let max_time_wait_for_seek = Duration::from_millis(AUDIO_TASK_POLL_FREQ_MS * 2);
        let start = Instant::now();
        while audio_state.lock().unwrap().events_received == event_count_before {
            tokio::time::sleep(Duration::from_millis(AUDIO_GET_SEEK_POLL_FREQ_MS)).await;

            if start.elapsed() >= max_time_wait_for_seek {
                warn!("Timed out waiting for audio thread getting 'seek' !");
                return 0.0;
            }
        }
        debug!("received one audioevent: assuming it's the SeekIs response to our GetSeek and that current_seek got updated");
        return audio_state.lock().unwrap().current_seek;
    }

    pub fn set_seek(&mut self, seek: f32) {
        let was_paused = self.paused;
        if !was_paused {
            audio_thread_send_cmd(AudioCommand::Pause, &self.cmd_tx);
        }
        audio_thread_send_cmd(AudioCommand::Seek(seek), &self.cmd_tx);
        if !was_paused {
            audio_thread_send_cmd(AudioCommand::Play, &self.cmd_tx);
        }
    }
}

pub struct AudioTask {
    queue_task_handle: JoinHandle<()>, // waits on song completion and send the audio thread the
    // new song to play
    audio_thread: AudioThread,
    pub state: Arc<Mutex<AudioState>>,
}

const AUDIO_TASK_POLL_FREQ_MS: u64 = 200;

impl AudioTask {
    pub fn run() -> Self {
        let cmd_queue = channel(10);
        let event_queue = channel(10);

        let (cmd_tx, cmd_rx) = cmd_queue;
        let (event_tx, event_rx) = event_queue;

        let state = Arc::new(Mutex::new(AudioState {
            current_song: None,
            paused: true,
            current_seek: 0.0,
            events_received: 0,
            cmd_tx,
            queue_future: VecDeque::new(),
            queue_past: VecDeque::new(),
        }));

        let audio_thread = start_audio_thread(cmd_rx, event_tx);
        let queue_task_handle = spawn(handle_audio_event(event_rx, state.clone()));

        Self {
            queue_task_handle,
            audio_thread,
            state,
        }
    }

    pub fn stop(self) {
        debug!("audio thread stop received");
        self.state
            .lock()
            .unwrap()
            .cmd_tx
            .send(AudioCommand::Quit)
            .unwrap();
        debug!("audio thread stop sent");
        stop_audio_thread(self.audio_thread);
        debug!("audio thread stopped");
        self.queue_task_handle.abort();
        debug!("audio queue cleaned");
    }
}

async fn handle_audio_event(mut rx: Receiver<AudioEvent>, state: Arc<Mutex<AudioState>>) {
    let mut interval = time::interval(Duration::from_millis(AUDIO_TASK_POLL_FREQ_MS));

    loop {
        interval.tick().await;
        if let Ok(event) = rx.try_recv() {
            state.lock().unwrap().events_received += 1;
            match event {
                AudioEvent::Finished => {
                    debug!("finished song");
                    state.lock().unwrap().next()
                }
                AudioEvent::SeekIs(value) => {
                    debug!("set audiostate current seek to {value}");
                    state.lock().unwrap().current_seek = value
                }
                _ => debug!("event ??"),
            }
        }
    }
}
