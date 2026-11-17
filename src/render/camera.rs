use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
	view_proj: [[f32; 4]; 4],
	pos: [f32; 3],
	_pad0: f32,
}

impl CameraUniform {
	pub fn new() -> Self {
		Self {
			view_proj: Mat4::IDENTITY.to_cols_array_2d(),
			pos: Vec3::ZERO.to_array(),
			_pad0: 0_f32,
		}
	}
}
