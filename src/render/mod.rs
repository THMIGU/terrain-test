pub mod bindings;
pub mod camera;
pub mod context;
pub mod pipeline;
pub mod surface;
pub mod vertex;

use std::sync::Arc;

use anyhow::Context;
use winit::window::Window;

use crate::{
	error::AppResult,
	render::{bindings::Bindings, context::GpuContext, surface::SurfaceState},
};

pub struct Render {
	window: Arc<Window>,
	gpu_ctx: GpuContext,
	surface_state: SurfaceState,
	bindings: Bindings,
	pipeline: wgpu::RenderPipeline,
}

impl Render {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			flags: Default::default(),
			memory_budget_thresholds: Default::default(),
			backend_options: Default::default(),
			display: None,
		});

		let (surface_state, adapter) = SurfaceState::new(&instance, window.clone())
			.await
			.context("Failed to initialize surface")?;
		let gpu_ctx = GpuContext::new(&adapter)
			.await
			.context("Failed to initialize GPU context")?;

		let bindings = Bindings::new(&gpu_ctx.device);

		let pipeline = pipeline::create(&gpu_ctx.device, &bindings, &surface_state.config);

		Ok(Self {
			window,
			gpu_ctx,
			surface_state,
			bindings,
			pipeline,
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			self.surface_state
				.config
				.width = width;
			self.surface_state
				.config
				.height = height;

			self.surface_state
				.surface
				.configure(&self.gpu_ctx.device, &self.surface_state.config);
			self.surface_state.configured = true;
		}
	}
}
