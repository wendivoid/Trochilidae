#import bevy_pbr::mesh_functions::{
    get_world_from_local, mesh_position_local_to_clip,
    mesh_position_local_to_world, mesh_normal_local_to_world,
}
#import bevy_pbr::pbr_functions::prepare_world_normal;
#import bevy_pbr::mesh_view_bindings::globals;
#import bevy_pbr::view_transformations::position_world_to_clip;
#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    pbr_types::pbr_input_new,
    prepass_io::{FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{FragmentOutput},
    pbr_functions::{apply_pbr_lighting, alpha_discard, main_pass_post_lighting_processing},
    pbr_types::{STANDARD_MATERIAL_FLAGS_UNLIT_BIT, pbr_input_new},
}
#endif

struct PlantMaterial {
    time_scale: f32,
};
@group(2) @binding(0) var<uniform> material: PlantMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(7) color: vec4<f32>,
    @location(8) generation: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(7) color: vec4<f32>,
    @location(8) generation: f32
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var model = get_world_from_local(vertex.instance_index);
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.color = vertex.color;
    out.generation = vertex.generation;
    return out;
}
