use std::{
	sync::{Arc, Mutex},
	thread,
	time::{Duration, Instant},
};

use crow::{
	glutin::{
		event::{ElementState, Event, WindowEvent},
		event_loop::{ControlFlow, EventLoop},
		platform::desktop::EventLoopExtDesktop,
		window::WindowBuilder,
	},
	Context,
};

use crate::{
	atlas::Atlas, control_manager::ControlManager, normalise_to, World, FRAME_LEN, TILE_SIZE,
};

pub struct FrameManager {
	mouse_position: (i64, i64),
	board_size: (i64, i64),
	board_position: (i64, i64),
	event_loop: Option<EventLoop<()>>,
	context: Context,
	pub control_manager: Arc<Mutex<ControlManager>>,
	frame_count: u64,
	pub redraw: bool,
	atlas: Atlas,
	world: Arc<Mutex<World>>,
	last_frame: Instant,
}
// TODO: Confirm safety
unsafe impl Sync for FrameManager {}
unsafe impl Send for FrameManager {}

impl FrameManager {
	pub fn new(world: Arc<Mutex<World>>, control_manager: Arc<Mutex<ControlManager>>) -> Self {
		let event_loop = EventLoop::new();
		let mut context = Context::new(WindowBuilder::new(), &event_loop).unwrap(); // TODO: Error Management
		let atlas = Atlas::new(&mut context);
		Self {
			mouse_position: (0, 0),
			board_size: (0, 0),
			board_position: (0, 0),
			context,
			event_loop: Some(event_loop),
			control_manager,
			atlas,
			frame_count: 0,
			redraw: true,
			world,
			last_frame: Instant::now(),
		}
	}
	pub fn run_once(&mut self) {
		if let Some(mut event_loop) = self.event_loop.take() {
			event_loop.run_return(
				|event: Event<()>, _window_target, control_flow: &mut ControlFlow| {
					self.frame_run(event, control_flow);
				},
			);
			self.event_loop.replace(event_loop);
		} else {
			println!("Tried to run but didn't have access to event loop");
		}
	}
	fn frame_run(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				WindowEvent::CursorMoved { position, .. } => {
					self.mouse_position = (position.x as i64, position.y as i64)
				}
				WindowEvent::MouseInput {
					state: ElementState::Pressed,
					button,
					..
				} => {
					let (mouse_x, mouse_y) = self.mouse_position;
					let window_size = self.context.window().inner_size();
					let (window_x, window_y) =
						(window_size.width as i64, window_size.height as i64);
					let world = self.world.lock().unwrap();
					let player_pos = world.player.get_position();
					let (player_x, player_y) = player_pos;

					println!(
						"mouse: {:?}, window: {:?}, player: {:?}",
						self.mouse_position,
						window_size,
						(player_x * TILE_SIZE, player_y * TILE_SIZE)
					);

					let (rel_center_x, rel_center_y) = (
						(mouse_x - (window_x / 2)), // Mouse X is inverse to Display X
						((window_y / 2) - mouse_y),
					);
					let (tile_rel_center_x, tile_rel_center_y) =
						(rel_center_x / TILE_SIZE, rel_center_y / TILE_SIZE);
					let (world_x, world_y) = (
						(tile_rel_center_x / 2) + player_x,
						(tile_rel_center_y / 2) + player_y,
					); // no idea why but values are doubled

					if let Ok(mut control_manager) = self.control_manager.lock() {
						(*control_manager).click(button, (world_x, world_y));
					} else {
						println!("Received input but Control Manager was locked!");
					}
				}
				/*WindowEvent::KeyboardInput { input, .. } => {
					if input.state == ElementState::Pressed {
						if let Some(keycode) = input.virtual_keycode {
							match keycode {
								VirtualKeyCode::Right => position = (position.0 + 1, position.1),
								VirtualKeyCode::Left => position = (position.0 - 1, position.1),
								VirtualKeyCode::Up => position = (position.0, position.1 + 1),
								VirtualKeyCode::Down => position = (position.0, position.1 - 1),
								_ => (),
							}
							request_redraw = true;
						}
					}
				}*/
				_ => (),
			},
			Event::MainEventsCleared => self.context.window().request_redraw(),
			Event::RedrawRequested(_) => {
				thread::sleep(normalise_to(
					FRAME_LEN,
					self.last_frame.elapsed().as_millis() as u64,
				));
				self.last_frame = Instant::now();
				self.frame_count += 1;
				//println!("Frame {}", self.frame_count);
				self.redraw = true;
				if self.redraw {
					//println!("Drawing frame {}", self.frame_count);
					self.redraw = false;
					let mut surface = self.context.surface();
					self.context.clear_color(&mut surface, (0.0, 0.0, 0.0, 1.0));

					let window_size = self.context.window().inner_size();

					self.board_size = (
						(window_size.width / TILE_SIZE as u32) as i64,
						(window_size.height / TILE_SIZE as u32) as i64,
					);

					if let Ok(mut world) = self.world.lock() {
						let player_pos = world.player.get_position();
						self.board_position = (
							player_pos.0 - (self.board_size.0 / 2),
							player_pos.1 - (self.board_size.1 / 2),
						);
						(*world).load(self.board_size);
						world.draw(
							&mut self.context,
							&mut surface,
							&self.atlas.atlas,
							self.board_size,
						);
						world.player.draw(
							&mut self.context,
							&mut surface,
							&self.atlas.atlas,
							self.board_position,
						);
					} else {
						println!("Couldn't access world, it was locked.");
					}
					self.context.present(surface).unwrap();
				}
			}
			_ => (),
		}
	}
}
