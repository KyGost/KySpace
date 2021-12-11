use std::{
	sync::{
		Arc,
		Mutex,
	},
	time::Instant,
};

use crate::{
	control_manager::Action,
	world::tile::{
		Direction,
		TilePos,
	},
	ControlManager,
	World,
	TICK_LEN,
};

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
						MoveTo(pos) => {
							if let Ok(mut world) = self.world.lock() {
								let player_pos = *world.player.get_position();
								let distance = player_pos - &pos;
								if distance == (0, 0).into() {
									return {
										(*control_manager).complete_pending();
										world.player.stopped_moving();
									};
								} else {
									let direction = Direction::from(distance);
									println!("move by: {:?}", direction);
									world.player.move_by(direction.into());
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
