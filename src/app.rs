use winit::{
	application::ApplicationHandler,
	dpi::PhysicalSize,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
pub struct App {
	window: Option<Window>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window_attributes = WindowAttributes::default()
			.with_title("terrain-test")
			.with_inner_size(PhysicalSize::new(800, 600))
			.with_resizable(false);

		self.window = Some(
			event_loop
				.create_window(window_attributes)
				.unwrap(),
		);
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::RedrawRequested => {
				self.window
					.as_ref()
					.unwrap()
					.request_redraw();
			}
			_ => (),
		}
	}
}
