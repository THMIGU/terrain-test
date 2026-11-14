mod app;
mod error;
mod state;

use winit::event_loop::{ControlFlow, EventLoop};

use crate::{app::App, error::AppResult};

fn main() -> AppResult {
	env_logger::init();

	let event_loop = EventLoop::new()?;
	event_loop.set_control_flow(ControlFlow::Poll);

	let mut app = App::default();
	event_loop.run_app(&mut app)?;

	Ok(())
}
