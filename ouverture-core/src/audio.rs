mod output;
mod player;

use crate::music::song::Song;


use rc_event_queue::mpmc::EventQueue;

pub use player::{AudioThread, start_audio_thread, stop_audio_thread};

pub use player::{audio_thread_send_cmd, AudioCommand};

pub fn audio_thread_play_song(eq: &EventQueue<AudioCommand>, song: Song) {
    audio_thread_send_cmd(AudioCommand::PlayNew(song), &eq);
}

pub fn audio_thread_play(eq: &EventQueue<AudioCommand>) {
    audio_thread_send_cmd(AudioCommand::Play, &eq);
}

pub fn audio_thread_pause(eq: &EventQueue<AudioCommand>) {
    audio_thread_send_cmd(AudioCommand::Pause, &eq);
}

