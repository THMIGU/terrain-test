use std::sync::Arc;

use winit::window::Window;

use crate::error::AppResult;

pub struct State {
	window: Arc<Window>,
}

impl State {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		Ok(Self {
			window,
		})
	}

	pub fn resize(&mut self, _width: u32, _height: u32) {
		()
	}

	pub fn render(&mut self) {
		self.window.request_redraw();
	}
}
