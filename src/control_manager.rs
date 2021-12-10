use crow::glutin::event::MouseButton;

use crate::MOVE_TIME;

#[derive(Debug)]
pub struct ControlManager {
	pub pending_action: Option<PlayerAction>,
}
impl ControlManager {
	pub fn new() -> Self {
		Self {
			pending_action: None,
		}
	}
	pub fn get_pending(&mut self) -> &Option<PlayerAction> {
		&self.pending_action
	}
	pub fn complete_pending(&mut self) -> Option<PlayerAction> {
		self.pending_action.take()
	}
	pub fn click(&mut self, button: MouseButton, pos: (i64, i64)) {
		if button == MouseButton::Left {
			self.pending_action = Some(PlayerAction::new(Action::MoveTo(pos.0, pos.1)));
		}
	}
}

#[derive(Debug)]
pub enum Action {
	MoveTo(i64, i64),
}

#[derive(Debug)]
pub struct PlayerAction {
	pub action: Action,
	pub countdown: u64,
}
impl PlayerAction {
	pub fn new(action: Action) -> Self {
		let mut new = Self {
			action,
			countdown: 0,
		};
		new.reset_countdown();
		new
	}
	fn reset_countdown(&mut self) {
		use Action::*;
		self.countdown = match &self.action {
			MoveTo(..) => MOVE_TIME,
		};
	}
	pub fn tick(&mut self) -> bool {
		if self.countdown == 0 {
			self.reset_countdown();
			true
		} else {
			self.countdown -= 1;
			false
		}
	}
}