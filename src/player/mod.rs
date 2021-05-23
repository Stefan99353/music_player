use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use actix::Addr;
use anyhow::Error;
use chrono::Utc;
use rand::seq::SliceRandom;
use rodio::{PlayError, Source};
use serde::{Deserialize, Serialize};

use crate::models::tracks::PopulatedTrack;
use crate::ws::player::hub::WsPlayerHub;
use crate::ws::player::messages::{RodioCommand, RodioCommandMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RodioPlayerState {
    pub current_track: Option<PopulatedTrack>,
    pub paused: bool,
    pub volume: f32,
    pub time: u128,
}

pub struct RodioPlayer {
    sink: rodio::Sink,
    stream_handle: rodio::OutputStreamHandle,
    ws_hub: Option<Arc<Addr<WsPlayerHub>>>,
    prev_tracks: VecDeque<PopulatedTrack>,
    current_track: Option<PopulatedTrack>,
    next_tracks: VecDeque<PopulatedTrack>,
    seek_to: Option<Duration>,
    paused: bool,
    volume: f32,
    current_started_at: Option<u128>,
    current_paused_at: Option<Duration>,
}

#[allow(dead_code)]
impl RodioPlayer {
    pub fn new(device: rodio::Device) -> anyhow::Result<(Self, rodio::OutputStream)> {
        let (stream, stream_handle) = rodio::OutputStream::try_from_device(&device)?;
        let sink = rodio::Sink::try_new(&stream_handle)?;
        sink.set_volume(0.5);

        Ok((
            Self {
                sink,
                stream_handle,
                ws_hub: None,
                prev_tracks: VecDeque::new(),
                current_track: None,
                next_tracks: VecDeque::new(),
                seek_to: None,
                paused: false,
                volume: 0.5,
                current_started_at: None,
                current_paused_at: None,
            },
            stream,
        ))
    }

    pub fn set_ws_connections(&mut self, ws_conns: Arc<Addr<WsPlayerHub>>) {
        self.ws_hub = Some(ws_conns);
    }

    pub fn ping_ws(&self) {
        debug!("Send player state to ws connections");
        if let Some(ws_hub) = &self.ws_hub {
            ws_hub.do_send(RodioCommandMessage { command: RodioCommand::State });
        }
    }

    pub fn add_track(&mut self, track: PopulatedTrack) {
        debug!("Adding track '{}'", &track.title);
        self.next_tracks.push_back(track);
    }

    pub fn add_tracks(&mut self, tracks: Vec<PopulatedTrack>, shuffle: bool) {
        debug!("Adding {} tracks", tracks.len());
        let mut tracks = tracks;

        if shuffle {
            let mut rng = rand::thread_rng();
            tracks.shuffle(&mut rng);
        }

        self.next_tracks.extend(tracks.into_iter());
    }

    pub fn clear_queue(&mut self) {
        self.next_tracks = VecDeque::new();
        self.seek_to = None;
    }

    pub fn resume(&mut self) {
        debug!("Resume playback");
        if self.paused {
            self.paused = false;
            self.sink.play();

            if let Some(paused_at) = self.current_paused_at {
                let now = Utc::now().timestamp_millis() as u128;

                self.current_started_at = Some(now - paused_at.as_millis());
            }

            self.ping_ws();
        }
    }

    pub fn pause(&mut self) {
        debug!("Pause playback");
        if !self.paused {
            self.paused = true;
            self.sink.pause();

            if let Some(started) = self.current_started_at {
                let now = Utc::now().timestamp_millis() as u128;
                self.current_paused_at = Some(Duration::from_millis((now - started) as u64));
            }

            self.ping_ws();
        }
    }

    pub fn stop(&mut self) -> Result<(), PlayError> {
        debug!("Stop playback");
        self.sink.stop();

        self.sink = rodio::Sink::try_new(&self.stream_handle)?;
        self.sink.set_volume(self.volume);
        self.prev_tracks = VecDeque::new();
        self.current_track = None;
        self.next_tracks = VecDeque::new();
        self.seek_to = None;
        self.paused = false;
        self.current_started_at = None;
        self.current_paused_at = None;

        self.ping_ws();

        Ok(())
    }

    pub fn next(&mut self) -> Result<(), PlayError> {
        debug!("Skip next");
        self.sink.stop();

        self.sink = rodio::Sink::try_new(&self.stream_handle)?;
        self.sink.set_volume(self.volume);
        if self.paused {
            self.sink.pause();
        }

        // self.ping_ws();

        Ok(())
    }

    pub fn prev(&mut self) -> anyhow::Result<()> {
        debug!("Skip previous");
        if self.prev_tracks.is_empty() {
            return Err(Error::msg("No previous tracks available"));
        }

        self.sink.stop();

        if let Some(current_track) = &self.current_track {
            self.next_tracks.push_front(current_track.clone())
        }

        self.next_tracks.push_front(self.prev_tracks.pop_back().unwrap());

        self.sink = rodio::Sink::try_new(&self.stream_handle)?;
        self.sink.set_volume(self.volume);
        if self.paused {
            self.sink.pause();
        }

        // self.ping_ws();

        Ok(())
    }

    pub fn seek(&mut self, to: Duration) -> anyhow::Result<()> {
        debug!("Seek to {}", &to.as_secs());
        self.seek_to = Some(to);

        self.sink.stop();

        if let Some(current_track) = &self.current_track {
            self.next_tracks.push_front(current_track.clone())
        }

        self.sink = rodio::Sink::try_new(&self.stream_handle)?;
        self.sink.set_volume(self.volume);
        if self.paused {
            self.sink.pause();
        }

        self.ping_ws();

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.sink.len()
    }

    pub fn set_volume(&mut self, volume: f32) {
        debug!("Set volume to {}", volume);
        self.volume = volume;
        self.sink.set_volume(volume);

        self.ping_ws();
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn get_queue(&self) -> Vec<PopulatedTrack> {
        self.next_tracks.iter().cloned().collect()
    }

    pub fn get_state(&self) -> RodioPlayerState {
        debug!("Getting state");

        let time = if self.current_track.is_some() {
            if let (Some(paused_at), true) = (self.current_paused_at, self.paused) {
                paused_at.as_millis()
            } else if let Some(started) = self.current_started_at {
                let now = Utc::now().timestamp_millis() as u128;
                now - started
            } else {
                0
            }
        } else {
            0
        };


        RodioPlayerState {
            current_track: self.current_track.clone(),
            paused: self.paused,
            volume: self.volume,
            time,
        }
    }
}

impl RodioPlayer {
    pub fn start_player(player: Arc<Mutex<RodioPlayer>>) -> JoinHandle<()> {
        debug!("Starting player thread cycle");
        thread::spawn(move || {
            loop {
                let track_found = player_cycle(player.clone()).unwrap();

                if track_found {
                    let _ = wait_till_end(player.clone()).join();

                    let mut player = player.lock().unwrap();
                    if let Some(prev_track) = player.current_track.clone() {
                        player.prev_tracks.push_back(prev_track);
                    }
                    player.current_track = None;
                    player.current_started_at = None;
                    player.current_paused_at = None;

                    player.ping_ws();
                }

                // Wait a while
                thread::sleep(std::time::Duration::from_millis(100));
            }
        })
    }
}

#[allow(dead_code)]
fn player_cycle(player: Arc<Mutex<RodioPlayer>>) -> anyhow::Result<bool> {
    let mut player = player.lock().unwrap();

    let track = player.next_tracks.pop_front();

    if let Some(next_track) = track {
        debug!("Found track {}", &next_track.title);
        let source = next_track.source()?;

        let mut now = Utc::now().timestamp_millis() as u128;

        if let Some(seek_to) = player.seek_to {
            let source = source.skip_duration(seek_to);
            now -= seek_to.as_millis();
            player.seek_to = None;
            player.sink.append(source);
        } else {
            player.sink.append(source);
        }

        player.current_track = Some(next_track);

        player.current_started_at = Some(now);

        player.ping_ws();

        Ok(true)
    } else {
        Ok(false)
    }
}

#[allow(dead_code)]
fn wait_till_end(player: Arc<Mutex<RodioPlayer>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let player = player.lock().unwrap();
        if player.sink.len() == 0 {
            break;
        }

        drop(player);

        thread::sleep(std::time::Duration::from_millis(250));
    })
}
