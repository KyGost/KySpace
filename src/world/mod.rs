use std::collections::HashMap;

use crow::{
	Context,
	Texture,
	WindowSurface,
};

use worldgen::{
	constraint,
	noise::perlin::PerlinNoise,
	noisemap::{
		NoiseMap,
		NoiseMapGenerator,
		Seed,
		Size,
		Step,
	},
	world::{
		tile::{
			Constraint,
			ConstraintType,
		},
		Tile,
		World as WorldMaker,
	},
};

use crate::{
	atlas::{
		Atlas,
		TextureType,
	},
	frame_manager::draw::Draw,
	player::Player,
	tile::*,
	CHUNK_X,
	CHUNK_Y,
};

use self::tile::{
	PixelPos,
	TilePos,
};

pub mod pixel_pos;
pub mod tile;
pub mod tile_pos;

#[derive(Clone)]
pub struct Chunk {
	tiles: Vec<Vec<(GroundType, ResourceType)>>,
}

pub struct World {
	groundmaker: WorldMaker<GroundType>,
	resourcemaker: WorldMaker<ResourceType>,
	chunks: HashMap<i64, HashMap<i64, Chunk>>,
	pub player: Player,
}
impl World {
	pub fn new(groundseed: &str, resourceseed: &str) -> Self {
		let noise = PerlinNoise::new();

		let groundmap = NoiseMap::new(noise)
			.set(Seed::of(groundseed))
			.set(Step::of(-0.1, -0.1));

		let resourcemap = NoiseMap::new(noise)
			.set(Seed::of(resourceseed))
			.set(Step::of(0.1, -0.1));

		let height_water = (-1.5, -0.5);
		let height_stillgrass = (-0.5, -0.4);
		let height_grass = (-0.4, 0.1);
		let height_windsweptgrass = (0.1, 0.4);
		let height_dirt = (0.4, 1.0);
		let height_stone = (1.0, 1.5);

		macro_rules! tile {
			($tile:expr, $height:expr) => {
				Tile::new($tile)
					.when(constraint!(Box::new(groundmap), > $height.0))
					.when(constraint!(Box::new(groundmap), < $height.1))
			};
			($tile:expr, $height:expr, $size:expr) => {
				tile!($tile, $height)
					.when(constraint!(Box::new(resourcemap), > $size.0))
					.when(constraint!(Box::new(resourcemap), < $size.1))
			};
		}

		let groundmaker = {
			use GroundType::*;
			WorldMaker::new()
				.set(Size::of(CHUNK_X as i64, CHUNK_Y as i64))
				.add(tile!(Water, height_water))
				.add(tile!(PlainGrass, height_stillgrass))
				.add(tile!(Grass, height_grass))
				.add(tile!(WindSweptGrass, height_windsweptgrass))
				.add(tile!(Dirt, height_dirt))
				.add(tile!(Stone, height_stone))
				.add(Tile::new(Dirt)) // Default dirt
		};

		let size_small = (0.0, 0.1);
		let size_medium = (0.1, 0.2);
		let size_large = (0.2, 0.35);

		let resourcemaker = {
			use ResourceType::*;
			WorldMaker::new()
				.set(Size::of(CHUNK_X as i64, CHUNK_Y as i64))
				.add(tile!(Rock, (height_dirt.0, height_stone.1), size_medium))
				.add(tile!(Bush, height_grass, size_medium))
				.add(tile!(Tree, height_grass, size_large))
				.add(tile!(Flower, height_stillgrass, size_small))
				.add(Tile::new(None))
		};

		Self {
			groundmaker,
			resourcemaker,
			chunks: HashMap::new(),
			player: Player::new(),
		}
	}
	pub fn load(&mut self, pos: TilePos, size: TilePos) {
		// Measure in chunks
		let chunk_pos = pos / &(CHUNK_X, CHUNK_Y).into();
		let chunk_size = size / &(CHUNK_X, CHUNK_Y).into();

		for chunk_x in chunk_pos.x - 1..chunk_pos.x + chunk_size.x {
			let mut row: HashMap<i64, Chunk> = self
				.chunks
				.get(&chunk_x)
				.map(|chunk| chunk.clone())
				.unwrap_or(HashMap::new());
			for chunk_y in chunk_pos.y - 1..chunk_pos.y + chunk_size.y {
				if !row.contains_key(&chunk_y) {
					let tiles = self
						.groundmaker
						.generate(chunk_x, chunk_y)
						.unwrap()
						.into_iter()
						.zip(self.resourcemaker.generate(chunk_x, chunk_y).unwrap())
						.map(|(ground, resource)| ground.into_iter().zip(resource).collect())
						.collect();
					row.insert(chunk_y, Chunk { tiles });
				}
			}
			self.chunks.insert(chunk_x, row);
		}
	}
	pub fn draw(
		&mut self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		atlas: &Atlas,
		pos: TilePos,
		size: TilePos,
		offset: PixelPos,
	) {
		// Measure in chunks
		let chunk_pos = pos / &(CHUNK_X, CHUNK_Y).into();
		let chunk_size = size / &(CHUNK_X, CHUNK_Y).into();

		for chunk_x in chunk_pos.x - 1..chunk_pos.x + chunk_size.x {
			let row = self.chunks.get(&chunk_x).unwrap();
			for chunk_y in chunk_pos.y - 1..chunk_pos.y + chunk_size.y {
				let chunk = row.get(&chunk_y).unwrap();
				chunk.tiles.iter().enumerate().for_each(|(col, tiles)| {
					tiles
						.iter()
						.enumerate()
						.for_each(|(row, (ground, resource))| {
							let chunk_pos: TilePos =
								TilePos::from((chunk_x, chunk_y)) * &(CHUNK_X, CHUNK_Y).into();
							let tile_pos: TilePos = chunk_pos
								+ &(col.try_into().unwrap(), row.try_into().unwrap()).into()
								- &pos;
							let pos = PixelPos::from(tile_pos) + &offset;
							ground.draw(ctx, surface, pos.clone(), atlas);
							resource.draw(ctx, surface, pos, atlas);
						})
				});
			}
		}
	}
}
