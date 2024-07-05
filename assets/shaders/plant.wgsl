#import bevy_pbr::mesh_functions::{
    get_model_matrix, mesh_position_local_to_clip,
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
    var model = get_model_matrix(vertex.instance_index);
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.color = vertex.color;
    out.generation = vertex.generation;
    return out;
}

struct FragmentInput {
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(7) color: vec4<f32>,
    @location(8) generation: f32
};

@fragment
fn fragment(
    in: FragmentInput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_new();
    pbr_input.material.base_color = in.color;
    pbr_input.world_normal = prepare_world_normal(
        in.world_normal,
        false,
        is_front
    );
    pbr_input.N = normalize(pbr_input.world_normal);

    // alpha discard
    //pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
#ifdef PREPASS_PIPELINE
    // write the gbuffer, lighting pass id, and optionally normal and motion_vector textures
    //let out = deferred_output(in, pbr_input);
#else
    // in forward mode, we calculate the lit color immediately, and then apply some post-lighting effects here.
    // in deferred mode the lit color and these effects will be calculated in the deferred lighting shader
    var out: FragmentOutput;
    if (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        out.color = apply_pbr_lighting(pbr_input);
    } else {
        out.color = pbr_input.material.base_color;
    }

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    if(in.generation * material.time_scale < globals.time) {
        out.color.a = 1.0;
    }

    return out;
}