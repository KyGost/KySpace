use std::collections::HashMap;

use crow::{Context, DrawConfig, Texture, WindowSurface};

use crate::atlas::{Atlas, OtherTexture, SpriteTexture, TextureType};
use crate::frame_manager::FrameManager;
use crate::tile::GroundType;
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
	moved_recently: bool,
	animation_frame: usize,
}
impl Player {
	pub fn new() -> Self {
		Self {
			position: (0, 0),
			facing: Direction::Up,
			moved_recently: false,
			animation_frame: 0,
		}
	}
	pub fn move_to(&mut self, pos: (i64, i64)) {
		self.position = pos;
	}
	pub fn stopped_moving(&mut self) {
		self.moved_recently = false;
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
		self.moved_recently = true;
	}
	pub fn get_position(&self) -> &(i64, i64) {
		&self.position
	}
	pub fn draw(
		&mut self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		atlas: &Atlas,
		size: (i64, i64),
		offset: (i64, i64),
	) {
		// Player position (center)
		//let (pos_x, pos_y) = self.get_position();
		// Board
		let (size_x, size_y) = size;
		//let (board_x, board_y) = board_position;

		let (pos_x, pos_y) = (size_x / 2, size_y / 2);
		let (pos_x, pos_y) = (pos_x / 2, pos_y / 2); // No idea why but we need to halve these
		let (pos_x, pos_y) = (pos_x * TILE_SIZE, pos_y * TILE_SIZE);
		let (pos_x, pos_y) = (pos_x + offset.0, pos_y + offset.1);

		let position_pixels = (pos_x as i32, pos_y as i32);

		if self.moved_recently {
			let textures = atlas
				.atlas
				.get(match self.facing {
					Direction::Up => &TextureType::AnimatedOther(OtherTexture::PlayerUp),
					Direction::Down => &TextureType::AnimatedOther(OtherTexture::PlayerDown),
					Direction::Left => &TextureType::AnimatedOther(OtherTexture::PlayerLeft),
					Direction::Right => &TextureType::AnimatedOther(OtherTexture::PlayerRight),
				})
				.unwrap();
			if let SpriteTexture::Animated(textures) = textures {
				ctx.draw(
					surface,
					&textures[self.animation_frame],
					position_pixels,
					&DrawConfig {
						scale: (2, 2),
						..DrawConfig::default()
					},
				);
				self.animation_frame = if self.animation_frame == textures.len() - 1 {
					0
				} else {
					self.animation_frame + 1
				};
			} else {
				unreachable!()
			}
		} else {
			let texture = atlas
				.atlas
				.get(match self.facing {
					Direction::Up => &TextureType::Other(OtherTexture::PlayerUp),
					Direction::Down => &TextureType::Other(OtherTexture::PlayerDown),
					Direction::Left => &TextureType::Other(OtherTexture::PlayerLeft),
					Direction::Right => &TextureType::Other(OtherTexture::PlayerRight),
				})
				.unwrap();
			match texture {
				SpriteTexture::Still(texture) => ctx.draw(
					surface,
					texture,
					position_pixels,
					&DrawConfig {
						scale: (2, 2),
						..DrawConfig::default()
					},
				),
				_ => unreachable!(),
			}
		}
	}
}
