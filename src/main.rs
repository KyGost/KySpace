use std::{
	collections::HashMap,
	thread,
	time::{Duration, Instant},
};

use crow::{
	glutin::{
		dpi::LogicalSize,
		event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
		event_loop::{ControlFlow, EventLoop},
		window::WindowBuilder,
	},
	target::{Offset, Scaled},
	Context, DrawConfig, DrawTarget, Texture, WindowSurface,
};

use worldgen::world::tile::{Constraint, ConstraintType};
use worldgen::{
	constraint,
	noise::perlin::PerlinNoise,
	noisemap::{NoiseMap, NoiseMapGenerator, NoiseMapGeneratorBase, Property, Seed, Size, Step},
	world::{Tile, World as WorldMaker},
};

const TILE_SIZE: usize = 32;
const CHUNK_X: usize = 4;
const CHUNK_Y: usize = 4;

fn main() -> Result<(), crow::Error> {
	let mut world = World::new();
	let mut position = (0, 0);

	let event_loop = EventLoop::new();
	let mut ctx = Context::new(WindowBuilder::new(), &event_loop)?;

	let atlas_texture = Texture::load(&mut ctx, "src/atlas.png").unwrap();
	let atlas_positions = HashMap::from([
		("stone", (10, 17)),
		("dirt", (11, 18)),
		("grass", (3, 18)),
		("water", (52, 21)),
		("tree", (14, 4)),
		("bush", (14, 3)),
		("rock", (8, 13)),
	]);
	let atlas = atlas_positions
		.into_iter()
		.map(|(name, (x, y))| {
			let texture = atlas_texture.get_section(
				(TILE_SIZE as u32 * x, TILE_SIZE as u32 * y),
				(TILE_SIZE as u32, TILE_SIZE as u32),
			);
			(String::from(name), texture)
		})
		.collect::<HashMap<String, Texture>>();

	let mut fps = FrameRateLimiter::new(20);
	event_loop.run(
		move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				WindowEvent::CursorMoved { position, .. } => {}
				WindowEvent::KeyboardInput { input, .. } => {
					if input.state == ElementState::Pressed {
						if let Some(keycode) = input.virtual_keycode {
							match keycode {
								VirtualKeyCode::Right => position = (position.0 + 1, position.1),
								VirtualKeyCode::Left => position = (position.0 - 1, position.1),
								VirtualKeyCode::Up => position = (position.0, position.1 + 1),
								VirtualKeyCode::Down => position = (position.0, position.1 - 1),
								_ => (),
							}
						}
					}
				}
				_ => (),
			},
			Event::MainEventsCleared => ctx.window().request_redraw(),
			Event::RedrawRequested(_) => {
				let mut surface = ctx.surface();
				ctx.clear_color(&mut surface, (0.0, 0.0, 0.0, 1.0));

				let window_size = ctx.window().inner_size();

				let size = (
					(window_size.width / TILE_SIZE as u32) as i64 / CHUNK_X as i64,
					(window_size.height / TILE_SIZE as u32) as i64 / CHUNK_Y as i64,
				);

				world.load(position, size);
				world.draw(&mut ctx, &mut surface, &atlas, position, size);
				ctx.present(surface).unwrap();
			}
			Event::RedrawEventsCleared => fps.frame(),
			_ => (),
		},
	)
}

#[derive(Clone, Debug)]
pub enum TileType {
	Stone,
	Dirt,
	Grass,
	Water,
	Tree,
	Bush,
	Rock,
}
impl TileType {
	fn draw(
		&self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		col: usize,
		row: usize,
		atlas: &HashMap<String, Texture>,
	) {
		use TileType::*;
		match self {
			Stone | Dirt | Grass => ctx.draw(
				surface,
				match self {
					Stone => atlas.get("stone").unwrap(),
					Dirt => atlas.get("dirt").unwrap(),
					Grass => atlas.get("grass").unwrap(),
					_ => unreachable!(),
				},
				((col * TILE_SIZE) as i32, (row * TILE_SIZE) as i32),
				&DrawConfig::default(),
			),
			Water => ctx.draw(
				surface,
				atlas.get("water").unwrap(),
				((col * TILE_SIZE) as i32, (row * TILE_SIZE) as i32),
				&DrawConfig::default(),
			),
			Tree | Bush | Rock => {
				ctx.draw(
					surface,
					atlas.get("grass").unwrap(),
					((col * TILE_SIZE) as i32, (row * TILE_SIZE) as i32),
					&DrawConfig::default(),
				);
				ctx.draw(
					surface,
					match self {
						Tree => atlas.get("tree").unwrap(),
						Bush => atlas.get("bush").unwrap(),
						Rock => atlas.get("rock").unwrap(),
						_ => unreachable!(),
					},
					((col * TILE_SIZE) as i32, (row * TILE_SIZE) as i32),
					&DrawConfig::default(),
				)
			}
		}
	}
}

#[derive(Clone)]
pub struct Chunk {
	tiles: Vec<Vec<TileType>>,
}

pub struct World {
	maker: WorldMaker<TileType>,
	chunks: HashMap<i64, HashMap<i64, Chunk>>,
}
impl World {
	fn new() -> Self {
		let noise = PerlinNoise::new();

		let nm = NoiseMap::new(noise)
			.set(Seed::of("Hello World!!"))
			.set(Step::of(0.2, 0.2));

		use TileType::*;
		let maker = WorldMaker::new()
			.set(Size::of(CHUNK_X as i64, CHUNK_Y as i64))
			.add(Tile::new(Water).when(constraint!(Box::new(nm), < -0.8)))
			.add(Tile::new(Grass).when(constraint!(Box::new(nm), < 0.2)))
			.add(Tile::new(Bush).when(constraint!(Box::new(nm), < 0.23)))
			.add(Tile::new(Tree).when(constraint!(Box::new(nm), < 0.26)))
			.add(Tile::new(Grass).when(constraint!(Box::new(nm), < 0.4)))
			.add(Tile::new(Dirt).when(constraint!(Box::new(nm), < 0.7)))
			.add(Tile::new(Rock).when(constraint!(Box::new(nm), < 0.71)))
			.add(Tile::new(Stone).when(constraint!(Box::new(nm), > 0.9)))
			.add(Tile::new(Dirt));

		Self {
			maker,
			chunks: HashMap::new(),
		}
	}
	fn load(&mut self, position: (i64, i64), size: (i64, i64)) {
		let (pos_x, pos_y) = position;
		let (size_x, size_y) = size;

		for chunk_x in pos_x..pos_x + size_x {
			let mut row: HashMap<i64, Chunk> = self
				.chunks
				.get(&chunk_x)
				.map(|chunk| chunk.clone())
				.unwrap_or(HashMap::new());
			for chunk_y in pos_y..pos_y + size_y {
				if !row.contains_key(&chunk_y) {
					let tiles = self.maker.generate(chunk_x, chunk_y).unwrap();
					row.insert(chunk_y, Chunk { tiles });
				}
			}
			self.chunks.insert(chunk_x, row);
		}
	}
	fn draw(
		&mut self,
		ctx: &mut Context,
		surface: &mut WindowSurface,
		atlas: &HashMap<String, Texture>,
		position: (i64, i64),
		size: (i64, i64),
	) {
		let (pos_x, pos_y) = position;
		let (size_x, size_y) = size;
		for chunk_x in pos_x..pos_x + size_x {
			let row = self.chunks.get(&chunk_x).unwrap();
			for chunk_y in pos_y..pos_y + size_y {
				let chunk = row.get(&chunk_y).unwrap();
				chunk.tiles.iter().enumerate().for_each(|(col, tiles)| {
					tiles.iter().enumerate().for_each(|(row, tile)| {
						tile.draw(
							ctx,
							surface,
							((chunk_x - pos_x) as usize * CHUNK_X) + col,
							((chunk_y - pos_y) as usize * CHUNK_Y) + row,
							atlas,
						);
					})
				});
			}
		}
	}
}

pub struct FrameRateLimiter {
	start: Instant,
	frame_count: u32,
	fps: u32,
}

impl FrameRateLimiter {
	pub fn new(fps: u32) -> Self {
		Self {
			start: Instant::now(),
			frame_count: 0,
			fps,
		}
	}

	pub fn frame(&mut self) {
		self.frame_count += 1;
		let finish = Duration::from_micros(1_000_000 / u64::from(self.fps)) * self.frame_count;
		if self.start.elapsed() < finish {
			while self.start.elapsed() < finish {
				thread::yield_now();
			}
		} else {
			println!("Lag at frame {}", self.frame_count)
		}
	}
}
