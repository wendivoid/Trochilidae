#define_import_path plants::vascular::vertex_output

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) generation: f32,
    @location(4) birthdate: f32,
};
