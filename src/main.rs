mod app;
mod assets;
mod camera;
mod error;
mod render;
mod shaders;
mod state;
mod vertex;

use anyhow::Context;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{app::App, error::AppResult};

fn main() -> AppResult {
	env_logger::init();

	let event_loop = EventLoop::new().context("Failed to create event loop")?;
	event_loop.set_control_flow(ControlFlow::Poll);

	let mut app = App::default();
	event_loop
		.run_app(&mut app)
		.context("Failed to run app")?;

	Ok(())
}
