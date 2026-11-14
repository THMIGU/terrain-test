use std::sync::Arc;

use winit::{
	application::ApplicationHandler,
	dpi::PhysicalSize,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{WindowAttributes, WindowId},
};

use crate::state::State;

#[derive(Default)]
pub struct App {
	state: Option<State>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window_attributes = WindowAttributes::default()
			.with_title("terrain-test")
			.with_inner_size(PhysicalSize::new(800, 600))
			.with_resizable(false);

		let window = Arc::new(
			event_loop
				.create_window(window_attributes)
				.unwrap(),
		);

		self.state = Some(pollster::block_on(State::new(window)).unwrap());
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		let Some(state) = &mut self.state else {
			return;
		};

		match event {
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::Resized(size) => state.resize(size.width, size.height),
			WindowEvent::RedrawRequested => state.render(),
			_ => (),
		}
	}
}
