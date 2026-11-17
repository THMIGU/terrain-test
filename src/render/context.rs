use anyhow::Context;

use crate::error::AppResult;

pub struct GpuContext {
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
}

impl GpuContext {
	pub async fn new(adapter: &wgpu::Adapter) -> AppResult<Self> {
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

		Ok(Self {
			device,
			queue,
		})
	}
}
