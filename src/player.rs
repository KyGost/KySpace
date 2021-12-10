use std::collections::HashMap;

use crow::{Context, DrawConfig, Texture, WindowSurface};

use crate::atlas::{OtherTexture, TextureType};
use crate::TILE_SIZE;

#[derive(Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Clone)]
pub struct Player {
	position: (i64, i64),
	facing: Direction,
}
impl Player {
	pub fn new() -> Self {
		Self {
			position: (0, 0),
			facing: Direction::Up,
		}
	}
	pub fn move_to(&mut self, pos: (i64, i64)) {
		self.position = pos;
	}
	pub fn move_by(&mut self, pos: (i64, i64)) {
		self.facing = if pos.1 > 0 {
			Direction::Up
		} else if pos.1 < 0 {
			Direction::Down
		} else if pos.0 < 0 {
			Direction::Left
		} else if pos.0 > 0 {
			Direction::Right
		} else {
			self.facing.clone() // TODO: Don't bother setting
		};
		self.position = (self.position.0 + pos.0, self.position.1 + pos.1);
	}
	pub fn get_position(&self) -> &(i64, i64) {
		&self.position
	}
	pub fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		atlas: &HashMap<TextureType, Texture>,
		board_position: (i64, i64),
	) {
		let player_texture = atlas
			.get(match self.facing {
				Direction::Up => &TextureType::Other(OtherTexture::PlayerUp),
				Direction::Down => &TextureType::Other(OtherTexture::PlayerDown),
				Direction::Left => &TextureType::Other(OtherTexture::PlayerLeft),
				Direction::Right => &TextureType::Other(OtherTexture::PlayerLeft),
			})
			.unwrap();
		let position_pixels = (
			((self.position.0 - board_position.0) * TILE_SIZE) as i32 / 2,
			((self.position.1 - board_position.1) * TILE_SIZE) as i32 / 2,
		); // Needs to be halved for some reason
		println!("player pos: {:?}", position_pixels);
		ctx.draw(
			surface,
			player_texture,
			position_pixels,
			&DrawConfig::default(),
		)
	}
}
