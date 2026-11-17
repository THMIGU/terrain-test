use anyhow::Context;
use image::GenericImageView;

use crate::{
	assets::TextureFiles,
	error::AppResult,
	render::{bindings::Bindings, context::GpuContext},
};

pub struct Texture {
	#[allow(unused)]
	pub texture: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub sampler: wgpu::Sampler,
	pub binding: wgpu::BindGroup,
}

impl Texture {
	pub fn from_image(
		gpu_ctx: &GpuContext,
		bindings: &Bindings,
		img: &image::DynamicImage,
		label: Option<&str>,
	) -> AppResult<Self> {
		let rgba = img.to_rgba8();
		let dimensions = img.dimensions();

		let size = wgpu::Extent3d {
			width: dimensions.0,
			height: dimensions.1,
			depth_or_array_layers: 1,
		};
		let texture = gpu_ctx
			.device
			.create_texture(&wgpu::TextureDescriptor {
				label,
				size,
				mip_level_count: 1,
				sample_count: 1,
				dimension: wgpu::TextureDimension::D2,
				format: wgpu::TextureFormat::Rgba8UnormSrgb,
				usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
				view_formats: &[],
			});

		gpu_ctx.queue.write_texture(
			wgpu::TexelCopyTextureInfo {
				texture: &texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
				aspect: wgpu::TextureAspect::All,
			},
			&rgba,
			wgpu::TexelCopyBufferLayout {
				offset: 0,
				bytes_per_row: Some(4 * dimensions.0),
				rows_per_image: Some(dimensions.1),
			},
			size,
		);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = gpu_ctx
			.device
			.create_sampler(&wgpu::SamplerDescriptor {
				address_mode_u: wgpu::AddressMode::ClampToEdge,
				address_mode_v: wgpu::AddressMode::ClampToEdge,
				address_mode_w: wgpu::AddressMode::ClampToEdge,
				mag_filter: wgpu::FilterMode::Linear,
				min_filter: wgpu::FilterMode::Nearest,
				mipmap_filter: wgpu::MipmapFilterMode::Nearest,
				..Default::default()
			});

		let binding = gpu_ctx
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: &bindings.texture_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: wgpu::BindingResource::TextureView(&view),
					},
					wgpu::BindGroupEntry {
						binding: 1,
						resource: wgpu::BindingResource::Sampler(&sampler),
					},
				],
				label: Some(&format!("{} Bind Group", label.unwrap_or("Texture"))),
			});

		Ok(Self {
			texture,
			view,
			sampler,
			binding,
		})
	}

	pub fn from_bytes(
		gpu_ctx: &GpuContext,
		bindings: &Bindings,
		bytes: &[u8],
		label: Option<&str>,
	) -> AppResult<Self> {
		let img = image::load_from_memory(bytes).context("Failed to load image from bytes")?;
		Self::from_image(gpu_ctx, bindings, &img, label)
	}

	pub fn from_file(
		gpu_ctx: &GpuContext,
		bindings: &Bindings,
		filename: &str,
		label: Option<&str>,
	) -> AppResult<Self> {
		let asset = TextureFiles::get(filename).context("Failed to load file")?;
		Self::from_bytes(gpu_ctx, bindings, &asset.data, label)
	}
}
