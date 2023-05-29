mod output;
mod player;

use tokio::task::spawn;

use log::debug;
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::music::song::Song;

use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

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
    current_song: Option<Song>,
    paused: bool,
    queue_future: VecDeque<Song>,
    queue_past: VecDeque<Song>,
}

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
        self.state
            .lock()
            .unwrap()
            .cmd_tx
            .send(AudioCommand::Quit)
            .unwrap();
        stop_audio_thread(self.audio_thread);
        self.queue_task_handle.abort();
    }
}

async fn handle_audio_event(mut rx: Receiver<AudioEvent>, state: Arc<Mutex<AudioState>>) {
    let mut interval = time::interval(Duration::from_millis(AUDIO_TASK_POLL_FREQ_MS));

    loop {
        interval.tick().await;
        if let Ok(event) = rx.try_recv() {
            match event {
                AudioEvent::Finished => state.lock().unwrap().next(),
                _ => debug!("event ??"),
            }
        }
    }
}
