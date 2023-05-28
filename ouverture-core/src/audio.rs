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

#[derive(Clone)]
pub struct AudioState {
    pub cmd_tx: Sender<AudioCommand>,
    current_song: Option<Song>,
    paused: bool,
    queue: VecDeque<Song>,
}


impl AudioState {
    pub fn play(&mut self, opt_song: Option<Song>) {
        if let Some(song) = opt_song {
            audio_thread_play_song(&self.cmd_tx, song.clone());
            self.current_song = Some(song);
        } else {
            audio_thread_play(&self.cmd_tx);
        }
        self.paused = false;
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

    pub fn next(&mut self) {
        let opt_song = self.queue.pop_front();
        if  opt_song.is_some() {
            self.play(opt_song);
        } else {
            self.current_song = None;
            self.paused = true;
        }
    }
}

pub struct AudioTask {
    queue_task_handle: JoinHandle<()>, // waits on song completion and send the audio thread the
    // new song to play
    audio_thread: AudioThread,
    pub state: Arc<Mutex<AudioState>>,
}

const AUDIO_TASK_POLL_FREQ_MS: u64 = 50;

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
            queue: VecDeque::new()
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
        self.state.lock().unwrap().cmd_tx.send(AudioCommand::Quit).unwrap();
        stop_audio_thread(self.audio_thread);
        self.queue_task_handle.abort();
    }
}

async fn handle_audio_event(mut rx: Receiver<AudioEvent>, state: Arc<Mutex<AudioState>>) {
    loop {
        if let Ok(event) = rx.try_recv() {
            match event {
                AudioEvent::Finished => debug!("event done"),
                _ => debug!("event ??"),
            }
        }
    }
}
