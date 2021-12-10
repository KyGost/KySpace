use crate::{
	atlas::{Atlas, SpriteTexture, TextureType},
	TILE_SIZE,
};
use crow::{Context, DrawConfig, WindowSurface};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum GroundType {
	Water,
	Grass,
	Dirt,
	Stone,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ResourceType {
	Rock,
	Bush,
	Tree,
	None,
}
impl GroundType {
	pub fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		x: i64,
		y: i64,
		offset: (i64, i64),
		atlas: &Atlas,
	) {
		let texture = atlas.atlas.get(&TextureType::Ground(self.clone())).unwrap();
		match texture {
			SpriteTexture::Still(texture) => ctx.draw(
				surface,
				texture,
				(
					((x * TILE_SIZE) + offset.0) as i32,
					((y * TILE_SIZE) + offset.1) as i32,
				),
				&DrawConfig {
					scale: (4, 4),
					..DrawConfig::default()
				},
			),
			_ => unimplemented!(),
		}
	}
}
impl ResourceType {
	pub fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		x: i64,
		y: i64,
		offset: (i64, i64),
		atlas: &Atlas,
	) {
		atlas
			.atlas
			.get(&TextureType::Resource(self.clone()))
			.map(|texture| match texture {
				SpriteTexture::Still(texture) => ctx.draw(
					surface,
					texture,
					(
						((x * TILE_SIZE) + offset.0) as i32,
						((y * TILE_SIZE) + offset.1) as i32,
					),
					&DrawConfig {
						scale: (4, 4),
						..DrawConfig::default()
					},
				),
				_ => unimplemented!(),
			});
	}
}
