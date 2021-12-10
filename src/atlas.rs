use crow::{Context, Texture};
use std::collections::HashMap;

use crate::{
	tile::{GroundType::*, ResourceType::*, *},
	TILE_SIZE,
};

pub enum SpriteTexture {
	Still(Texture),
	Animated(Vec<Texture>),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TextureType {
	Ground(GroundType),
	Resource(ResourceType),
	Other(OtherTexture),
	AnimatedOther(OtherTexture),
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum OtherTexture {
	PlayerUp,
	PlayerDown,
	PlayerLeft,
	PlayerRight,
}
use {OtherTexture::*, TextureType::*};

const TILE_SPRITES_SIZE: u32 = 32;
const TILE_SPRITES_ATLAS: &str = "src/atlas.png";
const TILE_SPRITES_POSITIONS: [(TextureType, (u32, u32)); 7] = [
	(Ground(Water), (52, 21)),
	(Ground(Grass), (3, 18)),
	(Ground(Dirt), (11, 18)),
	(Ground(Stone), (10, 17)),
	(Resource(Rock), (8, 13)),
	(Resource(Bush), (14, 3)),
	(Resource(Tree), (14, 4)),
];

const PLAYER_SPRITES_SIZE: u32 = 64;
const PLAYER_SPRITES_ATLAS: &str = "src/wulax_sprites/walkcycle/BODY_skeleton.png";
const PLAYER_SPRITES_POSITIONS: [(TextureType, (u32, u32)); 4] = [
	(Other(PlayerUp), (0, 3)),
	(Other(PlayerLeft), (0, 2)),
	(Other(PlayerDown), (0, 1)),
	(Other(PlayerRight), (0, 0)),
];
const ANIMATED_PLAYER_SPRITES_POSITIONS: [(TextureType, u32); 4] = [
	(AnimatedOther(PlayerUp), 3),
	(AnimatedOther(PlayerLeft), 2),
	(AnimatedOther(PlayerDown), 1),
	(AnimatedOther(PlayerRight), 0),
];

pub struct Atlas {
	pub atlas: HashMap<TextureType, SpriteTexture>,
}
impl Atlas {
	pub fn new(context: &mut Context) -> Self {
		let tile_atlas_texture = Texture::load(context, TILE_SPRITES_ATLAS).unwrap();
		let player_atlas_texture = Texture::load(context, PLAYER_SPRITES_ATLAS).unwrap();
		let atlas = TILE_SPRITES_POSITIONS
			.into_iter()
			.map(|(texture_type, (x, y))| {
				let texture = tile_atlas_texture.get_section(
					(TILE_SPRITES_SIZE * x, TILE_SPRITES_SIZE * y),
					(TILE_SPRITES_SIZE, TILE_SPRITES_SIZE),
				);
				(texture_type, SpriteTexture::Still(texture))
			})
			.chain(
				PLAYER_SPRITES_POSITIONS
					.into_iter()
					.map(|(texture_type, (x, y))| {
						let texture = player_atlas_texture.get_section(
							(x * PLAYER_SPRITES_SIZE, y * PLAYER_SPRITES_SIZE),
							(PLAYER_SPRITES_SIZE, PLAYER_SPRITES_SIZE),
						);
						(texture_type, SpriteTexture::Still(texture))
					}),
			)
			.chain(
				ANIMATED_PLAYER_SPRITES_POSITIONS
					.into_iter()
					.map(|(texture_type, y)| {
						let textures = (1..9)
							.into_iter()
							.map(|x| {
								player_atlas_texture.get_section(
									(x * PLAYER_SPRITES_SIZE, y * PLAYER_SPRITES_SIZE),
									(PLAYER_SPRITES_SIZE, PLAYER_SPRITES_SIZE),
								)
							})
							.collect();
						(texture_type, SpriteTexture::Animated(textures))
					}),
			)
			.collect::<HashMap<TextureType, SpriteTexture>>();
		Self { atlas }
	}
}
