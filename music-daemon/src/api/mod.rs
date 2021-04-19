use std::sync::Mutex;

use crate::player::RodioPlayer;

pub mod albums;
pub mod artists;
pub mod management;
pub mod player;
pub mod queue;
pub mod tracks;
pub mod images;

pub type PlayerData = Mutex<RodioPlayer>;
