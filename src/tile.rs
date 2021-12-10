use crate::{atlas::TextureType, TILE_SIZE};
use crow::{Context, DrawConfig, Texture, WindowSurface};
use std::collections::HashMap;

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
		atlas: &HashMap<TextureType, Texture>,
	) {
		let texture = atlas.get(&TextureType::Ground(self.clone())).unwrap();
		ctx.draw(
			surface,
			texture,
			((x * TILE_SIZE) as i32, (y * TILE_SIZE) as i32),
			&DrawConfig::default(),
		)
	}
}
impl ResourceType {
	pub fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		x: i64,
		y: i64,
		atlas: &HashMap<TextureType, Texture>,
	) {
		atlas
			.get(&TextureType::Resource(self.clone()))
			.map(|texture| {
				ctx.draw(
					surface,
					texture,
					((x * TILE_SIZE) as i32, (y * TILE_SIZE) as i32),
					&DrawConfig::default(),
				)
			});
	}
}
