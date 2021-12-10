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
}
impl Player {
	pub fn new() -> Self {
		Self {
			position: (0, 0),
			facing: Direction::Up,
			moved_recently: false,
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
		self.moved_recently = true;
	}
	pub fn get_position(&self) -> &(i64, i64) {
		&self.position
	}
	pub fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		atlas: &Atlas,
		board_position: (i64, i64),
		animations: &mut Vec<(u8, u8, Texture, Vec<Texture>, (i32, i32))>,
	) {
		let position_pixels = (
			((self.position.0 - board_position.0) * TILE_SIZE) as i32 / 2,
			((self.position.1 - board_position.1) * TILE_SIZE) as i32 / 2,
		); // Needs to be halved for some reason
		if self.moved_recently {
			let texture = atlas
				.atlas
				.get(match self.facing {
					Direction::Up => &TextureType::AnimatedOther(OtherTexture::PlayerUp),
					Direction::Down => &TextureType::AnimatedOther(OtherTexture::PlayerDown),
					Direction::Left => &TextureType::AnimatedOther(OtherTexture::PlayerLeft),
					Direction::Right => &TextureType::AnimatedOther(OtherTexture::PlayerRight),
				})
				.unwrap();
			let underlay = atlas
				.atlas
				.get(&TextureType::Ground(GroundType::Dirt))
				.unwrap();
			match (underlay, texture) {
				(SpriteTexture::Still(underlay), SpriteTexture::Animated(textures)) => {
					animations.push((0, 1, underlay.clone(), textures.clone(), position_pixels));
				}
				_ => unreachable!(),
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
