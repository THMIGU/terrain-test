use std::sync::Arc;

use anyhow::Context;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::error::AppResult;

pub struct State {
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	is_surface_configured: bool,
	window: Arc<Window>,
}

impl State {
	pub async fn new(window: Arc<Window>) -> AppResult<Self> {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			flags: Default::default(),
			memory_budget_thresholds: Default::default(),
			backend_options: Default::default(),
			display: None,
		});

		let surface = instance
			.create_surface(window.clone())
			.context("Failed to create surface")?;

		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
				apply_limit_buckets: false,
			})
			.await
			.context("Failed to request adapter")?;

		let (device, queue) = adapter
			.request_device(&wgpu::DeviceDescriptor {
				label: None,
				required_features: wgpu::Features::empty(),
				experimental_features: wgpu::ExperimentalFeatures::disabled(),
				required_limits: wgpu::Limits::default(),
				memory_hints: Default::default(),
				trace: wgpu::Trace::Off,
			})
			.await
			.context("Failed to request device")?;

		let surface_caps = surface.get_capabilities(&adapter);
		let surface_format = surface_caps
			.formats
			.iter()
			.find(|f| f.is_srgb())
			.copied()
			.unwrap_or(surface_caps.formats[0]);

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
			color_space: wgpu::SurfaceColorSpace::Auto,
		};

		println!("Adapter Info: {:#?}", adapter.get_info());

		Ok(Self {
			surface,
			device,
			queue,
			config,
			is_surface_configured: false,
			window,
		})
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			self.config.width = width;
			self.config.height = height;

			self.surface
				.configure(&self.device, &self.config);
			self.is_surface_configured = true;
		}
	}

	pub fn render(&mut self) {
		self.window.request_redraw();
	}

	pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
		match (code, is_pressed) {
			(KeyCode::Escape, true) => event_loop.exit(),
			_ => (),
		}
	}

	pub fn _update(&mut self) {
		todo!("Update shenanigans")
	}
}
