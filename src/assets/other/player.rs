use std::collections::HashMap;

use crow::{
	Context,
	DrawConfig,
	Texture,
	WindowSurface,
};

use crate::{
	atlas::{
		Atlas,
		OtherTexture,
		SpriteTexture,
		TextureType,
	},
	frame_manager::{
		Draw,
		FrameManager,
	},
	tile::GroundType,
	world::tile::TilePos,
	TILE_SIZE,
};

#[derive(Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Clone)]
pub struct Player {
	position: TilePos,
	facing: Direction,
	moved_recently: bool,
	animation_frame: usize,
}
impl Player {
	pub fn new() -> Self {
		Self {
			position: TilePos::from((0, 0)),
			facing: Direction::Up,
			moved_recently: false,
			animation_frame: 0,
		}
	}
	pub fn move_to(&mut self, pos: TilePos) {
		self.position = pos;
	}
	pub fn stopped_moving(&mut self) {
		self.moved_recently = false;
	}
	pub fn move_by(&mut self, dist: TilePos) {
		self.facing = if dist.y > 0 {
			Direction::Up
		} else if dist.y < 0 {
			Direction::Down
		} else if dist.x < 0 {
			Direction::Left
		} else if dist.x > 0 {
			Direction::Right
		} else {
			self.facing.clone() // TODO: Don't bother setting
		};
		self.position += &dist;
		self.moved_recently = true;
	}
	pub fn get_position(&self) -> &TilePos {
		&self.position
	}
}

impl Draw for Player {
	fn draw(
		&mut self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		atlas: &Atlas,
		pos: PixelPos,
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
