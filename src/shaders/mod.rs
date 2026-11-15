pub fn create_vertex_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
	device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some("Vertex Shader"),
		source: wgpu::ShaderSource::Wgsl(include_str!("vertex.wgsl").into()),
	})
}

pub fn create_fragment_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
	device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some("Fragment Shader"),
		source: wgpu::ShaderSource::Wgsl(include_str!("fragment.wgsl").into()),
	})
}
