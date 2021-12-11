use std::{
	sync::{
		Arc,
		Mutex,
	},
	thread,
	time::{
		Duration,
		Instant,
	},
};

use crow::{
	glutin::{
		dpi::PhysicalSize,
		event::{
			ElementState,
			Event,
			WindowEvent,
		},
		event_loop::{
			ControlFlow,
			EventLoop,
		},
		platform::desktop::EventLoopExtDesktop,
		window::WindowBuilder,
	},
	Context,
};

use crate::{
	atlas::Atlas,
	control_manager::ControlManager,
	normalise_to,
	world::tile::{
		PixelPos,
		TilePos,
	},
	World,
	FRAME_LEN,
	TILE_SIZE,
};

pub mod draw;

pub struct FrameManager {
	window_size: PhysicalSize<u32>,
	mouse_position: PixelPos,
	board_position: TilePos,
	board_size: TilePos, // Ideally should be differenciated...
	board_offset: PixelPos,
	event_loop: Option<EventLoop<()>>,
	context: Context,
	pub control_manager: Arc<Mutex<ControlManager>>,
	frame_count: u64,
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
			window_size: PhysicalSize {
				width: 0,
				height: 0,
			},
			mouse_position: (0, 0).into(),
			board_position: (0, 0).into(),
			board_size: (0, 0).into(),
			board_offset: (0, 0).into(),
			context,
			event_loop: Some(event_loop),
			control_manager,
			atlas,
			frame_count: 0,
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
				WindowEvent::Resized(new_size) => self.window_size = new_size,
				WindowEvent::CursorMoved { position, .. } => {
					self.mouse_position =
						PixelPos::from_mouse(position, self.context.window().inner_size());
				}
				WindowEvent::MouseInput {
					state: ElementState::Pressed,
					button,
					..
				} => {
					let clicked = self
						.mouse_position
						.to_rel_tile_pos(self.window_size)
						.to_world_tile(self.world.lock().unwrap().player.get_position());

					if let Ok(mut control_manager) = self.control_manager.lock() {
						(*control_manager).click(button, clicked);
					} else {
						println!("Received input but Control Manager was locked!");
					}
				}
				WindowEvent::KeyboardInput { input, .. } => {
					if input.state == ElementState::Pressed {
						if let Some(keycode) = input.virtual_keycode {
							if let Ok(mut control_manager) = self.control_manager.lock() {
								(*control_manager).press(
									keycode,
									*self.world.lock().unwrap().player.get_position(),
								);
							} else {
								println!("Received input but Control Manager was locked!");
							}
						}
					}
				}
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

				let mut surface = self.context.surface();
				self.context.clear_color(&mut surface, (0.0, 0.0, 0.0, 1.0));

				let window_size = self.context.window().inner_size();

				self.board_size = window_size.into();
				self.board_offset = (
					(window_size.width as i64 - (self.board_size.x * TILE_SIZE)) / 2,
					(window_size.height as i64 - (self.board_size.y * TILE_SIZE)) / 2,
				)
					.into(); // Get difference halved TODO: Use std ops

				if let Ok(mut world) = self.world.lock() {
					let player_pos = world.player.get_position();
					self.board_position = *player_pos - &(self.board_size / &TilePos::from((2, 2))); // TODO: Define differently
					(*world).load(self.board_position, self.board_size);

					world.draw(
						&mut self.context,
						&mut surface,
						&self.atlas,
						self.board_position,
						self.board_size,
						self.board_offset,
					);
					world.player.draw(
						&mut self.context,
						&mut surface,
						&self.atlas,
						self.board_size,
						self.board_offset,
					);
				} else {
					println!("Couldn't access world, it was locked.");
				}
				self.context.present(surface).unwrap();
			}
			_ => (),
		}
	}
}
