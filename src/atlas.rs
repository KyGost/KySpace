use crow::{Context, Texture};
use std::collections::HashMap;

use crate::{
	tile::{GroundType::*, ResourceType::*, *},
	TILE_SIZE,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TextureType {
	Ground(GroundType),
	Resource(ResourceType),
	Other(OtherTexture),
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum OtherTexture {
	PlayerUp,
	PlayerDown,
	PlayerLeft,
}
use {OtherTexture::*, TextureType::*};

const BASE_ATLAS_POSITIONS: [(TextureType, (u32, u32)); 7] = [
	(Ground(Water), (52, 21)),
	(Ground(Grass), (3, 18)),
	(Ground(Dirt), (11, 18)),
	(Ground(Stone), (10, 17)),
	(Resource(Rock), (8, 13)),
	(Resource(Bush), (14, 3)),
	(Resource(Tree), (14, 4)),
];

const OTHER_SPRITES: [(TextureType, &str); 3] = [
	(
		Other(PlayerUp),
		"src/tiny_rpg/sprites/hero/idle/hero-idle-back/hero-idle-back.png",
	),
	(
		Other(PlayerDown),
		"src/tiny_rpg/sprites/hero/idle/hero-idle-front/hero-idle-front.png",
	),
	(
		Other(PlayerLeft),
		"src/tiny_rpg/sprites/hero/idle/hero-idle-side/hero-idle-side.png",
	),
];

pub struct Atlas {
	pub atlas: HashMap<TextureType, Texture>,
}
impl Atlas {
	pub fn new(context: &mut Context) -> Self {
		let atlas_texture = Texture::load(context, "src/atlas.png").unwrap();
		let atlas = BASE_ATLAS_POSITIONS
			.into_iter()
			.map(|(texture_type, (x, y))| {
				let texture = atlas_texture.get_section(
					(TILE_SIZE as u32 * x, TILE_SIZE as u32 * y),
					(TILE_SIZE as u32, TILE_SIZE as u32),
				);
				(texture_type, texture)
			})
			.chain(OTHER_SPRITES.into_iter().map(|(texture_type, asset)| {
				(texture_type, Texture::load(context, asset).unwrap())
			}))
			.collect::<HashMap<TextureType, Texture>>();
		Self { atlas }
	}
}
