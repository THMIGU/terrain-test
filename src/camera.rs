use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

pub struct Camera {
	pub eye: Vec3,
	pub target: Vec3,
	pub up: Vec3,
	pub aspect: f32,
	pub fov_y: f32,
	pub z_near: f32,
	pub z_far: f32,
}

impl Camera {
	pub fn build_view_proj_matrix(&self) -> Mat4 {
		let view = glam::camera::rh::view::look_at_mat4(self.eye, self.target, self.up);
		let proj = glam::camera::rh::proj::directx::perspective(
			self.fov_y.to_degrees(),
			self.aspect,
			self.z_near,
			self.z_far,
		);

		proj * view
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
	view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
	pub fn new() -> Self {
		Self {
			view_proj: Mat4::IDENTITY.to_cols_array_2d(),
		}
	}

	pub fn update_view_proj(&mut self, camera: &Camera) {
		self.view_proj = camera
			.build_view_proj_matrix()
			.to_cols_array_2d();
	}
}
