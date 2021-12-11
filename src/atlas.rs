use {
	crow::{
		Context,
		Texture,
	},
	std::collections::HashMap,
};

use crate::{
	tile::{
		GroundType::*,
		ResourceType::*,
		*,
	},
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
	AnimatedGround(GroundType),
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum OtherTexture {
	PlayerUp,
	PlayerDown,
	PlayerLeft,
	PlayerRight,
}
use {
	OtherTexture::*,
	TextureType::*,
};

const TILE_SPRITE_SIZE: u32 = 32;
const TILE_SPRITE_ATLAS: &str = "src/bluecarrot16_sprites/terrain.png";
const TILE_SPRITE_POSITIONS: [(TextureType, (u32, u32)); 4] = [
	(Ground(Dirt), (8, 5)),
	(Ground(Stone), (2, 5)),
	(Ground(PlainGrass), (4, 28)),
	(Ground(Grass), (10, 5)),
];
const ANIMATED_TILE_SPRITE_POSITIONS: [(TextureType, (u32, u32)); 2] =
	[(Ground(Water), (9, 3)), (Ground(WindSweptGrass), (9, 5))];

const ROCK_SPRITE_SIZE: u32 = 32;
const ROCK_SPRITE_ATLAS: &str = "src/bluecarrot16_sprites/rocks.png";
const ROCK_SPRITE_POSITIONS: [(TextureType, (u32, u32)); 1] = [(Resource(Rock), (4, 11))];

const PLANT_SPRITE_SIZE: u32 = 32;
const PLANT_SPRITE_ATLAS: &str = "src/bluecarrot16_sprites/plants.png";
const PLANT_SPRITE_POSITIONS: [(TextureType, (u32, u32)); 3] = [
	(Resource(Bush), (13, 5)),
	(Resource(Tree), (11, 5)),
	(Resource(Flower), (10, 31)),
];

const PLAYER_SPRITE_SIZE: u32 = 64;
const PLAYER_SPRITE_ATLAS: &str = "src/wulax_sprites/walkcycle/BODY_skeleton.png";
const PLAYER_SPRITE_POSITIONS: [(TextureType, (u32, u32)); 4] = [
	(Other(PlayerUp), (0, 3)),
	(Other(PlayerLeft), (0, 2)),
	(Other(PlayerDown), (0, 1)),
	(Other(PlayerRight), (0, 0)),
];
const ANIMATED_PLAYER_SPRITE_POSITIONS: [(TextureType, u32); 4] = [
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
		let tile_atlas_texture = Texture::load(context, TILE_SPRITE_ATLAS).unwrap();
		let player_atlas_texture = Texture::load(context, PLAYER_SPRITE_ATLAS).unwrap();
		let plant_atlas_texture = Texture::load(context, PLANT_SPRITE_ATLAS).unwrap();
		let rock_atlas_texture = Texture::load(context, ROCK_SPRITE_ATLAS).unwrap();
		let atlas = TILE_SPRITE_POSITIONS
			.into_iter()
			.map(|(texture_type, (x, y))| {
				let texture = tile_atlas_texture.get_section(
					(TILE_SPRITE_SIZE * x, TILE_SPRITE_SIZE * y),
					(TILE_SPRITE_SIZE, TILE_SPRITE_SIZE),
				);
				(texture_type, SpriteTexture::Still(texture))
			})
			.chain(
				PLANT_SPRITE_POSITIONS
					.into_iter()
					.map(|(texture_type, (x, y))| {
						let texture = plant_atlas_texture.get_section(
							(x * PLANT_SPRITE_SIZE, y * PLANT_SPRITE_SIZE),
							(PLANT_SPRITE_SIZE, PLANT_SPRITE_SIZE),
						);
						(texture_type, SpriteTexture::Still(texture))
					}),
			)
			.chain(
				ROCK_SPRITE_POSITIONS
					.into_iter()
					.map(|(texture_type, (x, y))| {
						let texture = rock_atlas_texture.get_section(
							(x * ROCK_SPRITE_SIZE, y * ROCK_SPRITE_SIZE),
							(ROCK_SPRITE_SIZE, ROCK_SPRITE_SIZE),
						);
						(texture_type, SpriteTexture::Still(texture))
					}),
			)
			.chain(
				PLAYER_SPRITE_POSITIONS
					.into_iter()
					.map(|(texture_type, (x, y))| {
						let texture = player_atlas_texture.get_section(
							(x * PLAYER_SPRITE_SIZE, y * PLAYER_SPRITE_SIZE),
							(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE),
						);
						(texture_type, SpriteTexture::Still(texture))
					}),
			)
			.chain(
				ANIMATED_PLAYER_SPRITE_POSITIONS
					.into_iter()
					.map(|(texture_type, y)| {
						let textures = (1..9)
							.into_iter()
							.map(|x| {
								player_atlas_texture.get_section(
									(x * PLAYER_SPRITE_SIZE, y * PLAYER_SPRITE_SIZE),
									(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE),
								)
							})
							.collect();
						(texture_type, SpriteTexture::Animated(textures))
					}),
			)
			.chain(
				ANIMATED_TILE_SPRITE_POSITIONS
					.into_iter()
					.map(|(texture_type, (x, y))| {
						let textures = (0..3)
							.into_iter()
							.map(|f| {
								tile_atlas_texture.get_section(
									((x + f) * TILE_SPRITE_SIZE, y * TILE_SPRITE_SIZE),
									(TILE_SPRITE_SIZE, TILE_SPRITE_SIZE),
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
