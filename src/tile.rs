use {
	crate::{
		atlas::{
			Atlas,
			SpriteTexture,
			TextureType,
		},
		frame_manager::draw::Draw,
		world::tile::PixelPos,
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
	fn draw(&self, ctx: &mut Context, surface: &mut WindowSurface, pos: PixelPos, atlas: &Atlas) {
		let texture = atlas.atlas.get(&TextureType::Ground(self.clone())).unwrap();
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
	}
}
impl Draw for ResourceType {
	fn draw(&self, ctx: &mut Context, surface: &mut WindowSurface, pos: PixelPos, atlas: &Atlas) {
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
			});
	}
}
