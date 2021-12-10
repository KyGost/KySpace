use std::{
	sync::{Arc, Mutex},
	time::Instant,
};

use crate::{control_manager::Action, ControlManager, World, TICK_LEN};

pub struct TickManager {
	world: Arc<Mutex<World>>,
	control_manager: Arc<Mutex<ControlManager>>,
	last_tick: Instant,
	pub tick_gap: u128,
}
impl TickManager {
	pub fn new(world: Arc<Mutex<World>>, control_manager: Arc<Mutex<ControlManager>>) -> Self {
		Self {
			world,
			control_manager,
			last_tick: Instant::now(),
			tick_gap: TICK_LEN as u128,
		}
	}
	pub fn run_once(&mut self) {
		self.tick_gap = self.last_tick.elapsed().as_millis();
		self.last_tick = Instant::now();
		let mut control_manager = self.control_manager.lock().unwrap(); // TODO: Handle
		use Action::*;
		let pending_action = control_manager.pending_action.take();
		match pending_action {
			Some(mut player_action) => {
				if player_action.tick() {
					match player_action.action {
						MoveTo(x, y) => {
							if let Ok(mut world) = self.world.lock() {
								let pos = *world.player.get_position();
								let distance = (x - pos.0, y - pos.1);
								let move_by = (
									if distance.0 > 0 {
										1
									} else if distance.0 < 0 {
										-1
									} else {
										0
									},
									if distance.1 > 0 {
										1
									} else if distance.1 < 0 {
										-1
									} else {
										0
									},
								);
								if move_by == (0, 0) {
									return {
										(*control_manager).complete_pending();
									};
								} else {
									println!("move by: {:?}", move_by);
									world.player.move_by(move_by);
								}
							} else {
								println!("Couldn't move! World locked!");
							}
						}
					}
				}
				control_manager.pending_action = Some(player_action);
			}
			None => {}
		}
	}
}
// TODO: Confirm safety
unsafe impl Sync for TickManager {}
unsafe impl Send for TickManager {}
