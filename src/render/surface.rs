use std::sync::Arc;

use anyhow::Context;
use wgpu::Instance;
use winit::window::Window;

use crate::error::AppResult;

pub struct SurfaceState {
	pub surface: wgpu::Surface<'static>,
	pub config: wgpu::SurfaceConfiguration,
	pub configured: bool,
}

impl SurfaceState {
	pub async fn new(instance: &Instance, window: Arc<Window>) -> AppResult<(Self, wgpu::Adapter)> {
		let size = window.inner_size();

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

		let surface_caps = surface.get_capabilities(&adapter);
		let surface_format = surface_caps
			.formats
			.iter()
			.find(|f| f.is_srgb())
			.copied()
			.unwrap_or(surface_caps.formats[0]);

		let surface_config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: if surface_caps
				.present_modes
				.contains(&wgpu::PresentMode::Mailbox)
			{
				wgpu::PresentMode::Mailbox
			} else {
				wgpu::PresentMode::Fifo
			},
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
			color_space: wgpu::SurfaceColorSpace::Auto,
		};

		Ok((
			Self {
				surface,
				config: surface_config,
				configured: false,
			},
			adapter,
		))
	}
}
