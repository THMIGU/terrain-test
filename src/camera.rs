use glam::{Mat4, Quat, Vec3};

pub struct Camera {
	pub pos: Vec3,
	pub rot: Quat,
	pub aspect: f32,
	pub fov_y: f32,
	pub z_near: f32,
	pub z_far: f32,
}

impl Camera {
	pub fn build_view_proj_matrix(&self) -> Mat4 {
		let view = Mat4::from_quat(self.rot);
		let proj = glam::camera::rh::proj::directx::perspective(
			self.fov_y.to_radians(),
			self.aspect,
			self.z_near,
			self.z_far,
		);

		proj * view
	}
}
