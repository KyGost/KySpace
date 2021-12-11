use {
	crate::{
		atlas::{
			Atlas,
			OtherTexture,
			SpriteTexture,
			TextureType,
		},
		frame_manager::draw::Draw,
		world::tile::{
			PixelPos,
			TilePos,
		},
		Error,
	},
	crow::{
		Context,
		DrawConfig,
		WindowSurface,
	},
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
}
impl Player {
	pub fn new() -> Self {
		Self {
			position: TilePos::from((0, 0)),
			facing: Direction::Up,
			moved_recently: false,
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
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
		frame: usize,
	) -> Result<(), Error> {
		/*
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
		*/
		let pos = pos / 2; // TODO: Figure out why this is needed

		if self.moved_recently {
			self.draw_animated(ctx, surface, pos, atlas, frame)
		} else {
			self.draw_still(ctx, surface, pos, atlas)
		}
	}
	fn draw_still(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
	) -> Result<(), Error> {
		let texture = atlas
			.atlas
			.get(match self.facing {
				Direction::Up => &TextureType::Other(OtherTexture::PlayerUp),
				Direction::Down => &TextureType::Other(OtherTexture::PlayerDown),
				Direction::Left => &TextureType::Other(OtherTexture::PlayerLeft),
				Direction::Right => &TextureType::Other(OtherTexture::PlayerRight),
			})
			.ok_or(Error::MissingTexture)?;
		if let SpriteTexture::Still(texture) = texture {
			Ok(ctx.draw(
				surface,
				texture,
				pos.into(),
				&DrawConfig {
					scale: (2, 2),
					..DrawConfig::default()
				},
			))
		} else {
			Err(Error::MissingTexture)
		}
	}
	fn draw_animated(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
		frame: usize,
	) -> Result<(), Error> {
		let textures = atlas
			.atlas
			.get(match self.facing {
				Direction::Up => &TextureType::AnimatedOther(OtherTexture::PlayerUp),
				Direction::Down => &TextureType::AnimatedOther(OtherTexture::PlayerDown),
				Direction::Left => &TextureType::AnimatedOther(OtherTexture::PlayerLeft),
				Direction::Right => &TextureType::AnimatedOther(OtherTexture::PlayerRight),
			})
			.ok_or(Error::MissingTexture)?;
		if let SpriteTexture::Animated(textures) = textures {
			Ok(ctx.draw(
				surface,
				textures
					.get(frame % textures.len())
					.ok_or(Error::MissingTexture)?,
				pos.into(),
				&DrawConfig {
					scale: (2, 2),
					..DrawConfig::default()
				},
			))
		} else {
			Err(Error::MissingTexture)
		}
	}
}
