use {
	crate::{
		atlas::{
			Atlas,
			SpriteTexture,
			TextureType,
		},
		frame_manager::draw::Draw,
		world::tile::PixelPos,
		Error,
		TILE_SIZE,
	},
	crow::{
		Context,
		DrawConfig,
		WindowSurface,
	},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum GroundType {
	Water,
	PlainGrass,
	Grass,
	WindSweptGrass,
	Dirt,
	Stone,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ResourceType {
	Rock,
	Bush,
	Tree,
	Flower,
	None,
}
impl Draw for GroundType {
	fn draw_animated(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
		frame: usize,
	) -> Result<(), Error> {
		let texture = atlas
			.atlas
			.get(&TextureType::Ground(self.clone()))
			.ok_or(Error::MissingTexture)?;
		if let SpriteTexture::Animated(textures) = texture {
			Ok(ctx.draw(
				surface,
				&textures[frame % textures.len()],
				pos.into(),
				&DrawConfig {
					scale: (4, 4),
					..DrawConfig::default()
				},
			))
		} else {
			Err(Error::MissingTexture)
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
			.get(&TextureType::Ground(self.clone()))
			.ok_or(Error::MissingTexture)?;
		match texture {
			SpriteTexture::Still(texture) => ctx.draw(
				surface,
				texture,
				pos.into(),
				&DrawConfig {
					scale: (4, 4),
					..DrawConfig::default()
				},
			),
			SpriteTexture::Animated(textures) => ctx.draw(
				surface,
				&textures[fastrand::usize(..textures.len())],
				pos.into(),
				&DrawConfig {
					scale: (4, 4),
					..DrawConfig::default()
				},
			),
		}
		Ok(())
	}
}
impl Draw for ResourceType {
	fn draw_still(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		pos: PixelPos,
		atlas: &Atlas,
	) -> Result<(), Error> {
		atlas
			.atlas
			.get(&TextureType::Resource(self.clone()))
			.map(|texture| match texture {
				SpriteTexture::Still(texture) => ctx.draw(
					surface,
					texture,
					pos.into(),
					&DrawConfig {
						scale: (4, 4),
						..DrawConfig::default()
					},
				),
				_ => unimplemented!(),
			})
			.ok_or(Error::MissingTexture)
	}
}
