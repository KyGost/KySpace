use std::{
	sync::{
		Arc,
		Mutex,
	},
	thread,
	time::Duration,
};

mod frame_manager;
use frame_manager::FrameManager;
mod tick_manager;
use tick_manager::TickManager;
pub mod control_manager;
use control_manager::ControlManager;

mod pixel_tile_conversion;
pub use pixel_tile_conversion::*;

pub mod atlas;
pub mod tile;
pub mod world;
use world::World;

pub mod assets;

pub use world::tile::TILE_SIZE;

// Board
const CHUNK_X: i64 = 4; // Size of generation
const CHUNK_Y: i64 = 4; // Size of generation

// Frames
const FRAME_LEN: u64 = 80; // 20 ms :. 50fps

// Ticks
const TICK_LEN: u64 = 100; // 100 ms :. 10tps
const MOVE_TIME: u64 = 3; // :. 0.5s
						  //const CHOP_TIME: u64 = 20; // :. 2s

fn main() {
	let world = World::new("Wet", "Shiny");
	let control_manager = ControlManager::new();
	//let mut move_by: (i64, i64) = (0, 0);

	let world_arc = Arc::new(Mutex::new(world));
	let control_arc = Arc::new(Mutex::new(control_manager));

	let mut frame_manager = FrameManager::new(world_arc.clone(), control_arc.clone());
	let mut tick_manager = TickManager::new(world_arc, control_arc);

	thread::spawn(move || loop {
		tick_manager.run_once().unwrap();
		thread::sleep(normalise_to(TICK_LEN, tick_manager.tick_gap as u64));
	});
	loop {
		frame_manager.run_once() // For some reason render goes yuck if done from another thread
	}
}

pub fn normalise_to(aim_ms: u64, recent_ms: u64) -> Duration {
	Duration::from_millis(std::cmp::max(((aim_ms * 3) as i64 - recent_ms as i64) / 2, 0) as u64)
}

#[derive(Debug)]
pub enum Error {
	MissingTexture,
	AnimationUnimplemented,
	ControlManagerLocked,
	WorldManagerLocked,
}
