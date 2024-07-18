#import bevy_pbr::mesh_functions::{
    get_world_from_local,
    mesh_position_local_to_world,
    mesh_normal_local_to_world,
    mesh_position_local_to_clip
}
#import plants::vascular::vertex_output::VertexOutput

struct InstanceIndex {
    index: u32,
    // We have to respect the memory layout here
    _padding1: u32,
    _padding2: vec2u,
}

@group(2) @binding(0)
var<uniform> instance_index: InstanceIndex;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) m_color: vec4<f32>,
    @location(3) i_pos: vec4<f32>,
    @location(4) i_birthdate: f32,
    @location(5) m_generation: f32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var position = vertex.position * vertex.i_pos.w + vertex.i_pos.xyz;
    var out: VertexOutput;
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var world_from_local = get_world_from_local(instance_index.index);

    // NOTE: Passing 0 as the instance_index to get_world_from_local() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.
    out.position = mesh_position_local_to_clip(
        world_from_local,
        vec4<f32>(position, 0.0)
    );

    out.world_position = mesh_position_local_to_world(world_from_local, vec4<f32>(position, 0.0));

    out.world_normal = mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        instance_index.index
    );
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    //out.instance_index = vertex.instance_index;

    out.color = vertex.m_color;
    out.generation = vertex.m_generation;
    out.birthdate = vertex.i_birthdate;

    return out;
}