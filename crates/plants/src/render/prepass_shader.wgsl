#import bevy_pbr::{
    mesh_functions,
    skinning,
    morph,
    mesh_view_bindings::{view, previous_view_proj},
}

#ifdef DEFERRED_PREPASS
#import bevy_pbr::rgb9e5
#endif

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    //@location(3) normal: vec3<f32>,
    @location(7) color: vec4<f32>,
    @location(8) generation: f32
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    //@location(2) world_normal: vec3<f32>,
    @location(4) world_position: vec4<f32>,
    @location(8) color: vec4<f32>,
    @location(9) generation: f32
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    var model = mesh_functions::get_model_matrix(vertex.instance_index);

    out.position = mesh_functions::mesh_position_local_to_clip(model, vec4(vertex.position, 1.0));

    //out.world_normal = mesh_functions::mesh_normal_local_to_world(
    //    vertex.normal,
    //    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    //    // See https://github.com/gfx-rs/naga/issues/2416
    //    vertex.instance_index
    //);

    out.color = vertex.color;
    out.generation = vertex.generation;

    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));

    return out;
}

struct FragmentOutput {
    @location(0) normal: vec4<f32>,
#ifdef DEFERRED_PREPASS
    @location(2) deferred: vec4<u32>,
    @location(3) deferred_lighting_pass_id: u32,
    @location(9) generation: f32
#endif
}

struct PlantMaterial {
    time_scale: f32,
};
@group(2) @binding(0) var<uniform> material: PlantMaterial;

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    //out.normal = vec4(0.0, 0.0, 0.0, 0.0);

#ifdef DEFERRED_PREPASS
    // There isn't any material info available for this default prepass shader so we are just writing 
    // emissive magenta out to the deferred gbuffer to be rendered by the first deferred lighting pass layer.
    // This is here so if the default prepass fragment is used for deferred magenta will be rendered, and also
    // as an example to show that a user could write to the deferred gbuffer if they were to start from this shader.
    //out.deferred = vec4(0u, bevy_pbr::rgb9e5::vec3_to_rgb9e5_(vec3(1.0, 0.0, 0.0)), 0u, 0u);
    //out.deferred_lighting_pass_id = 1u;
#endif

    if(in.generation * material.time_scale > globals.time) {
        discard;
    }

    return out;
}