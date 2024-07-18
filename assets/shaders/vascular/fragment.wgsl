#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::pbr_types::{PbrInput, pbr_input_new};
#import bevy_pbr::pbr_functions::{alpha_discard, prepare_world_normal, calculate_view};
#import bevy_pbr::{mesh_view_bindings::view, mesh_bindings::mesh};
#import bevy_pbr::forward_io::FragmentOutput;
#import bevy_pbr::{
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND,
}
#import bevy_pbr::mesh_view_bindings::globals;
#import plants::vascular::vertex_output::VertexOutput;

fn pbr_input_from_vertex_output(
    in: VertexOutput,
    is_front: bool,
    double_sided: bool,
) -> PbrInput {
    var pbr_input: PbrInput = pbr_input_new();

    pbr_input.flags = STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND;
    pbr_input.material.base_color = in.color;

    pbr_input.is_orthographic = view.clip_from_view[3].w == 1.0;
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    pbr_input.material.alpha_cutoff = 0.5;

    pbr_input.world_normal = prepare_world_normal(
        in.world_normal,
        double_sided,
        is_front,
    );

    pbr_input.N = normalize(pbr_input.world_normal);

    return pbr_input;
}
@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {

    var pbr_input = pbr_input_from_vertex_output(in, is_front, false);

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    // in forward mode, we calculate the lit color immediately, and then apply some post-lighting effects here.
    // in deferred mode the lit color and these effects will be calculated in the deferred lighting shader
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // Only display the part of the mesh that was 'born' yet.
    if(in.generation > globals.time - in.birthdate) {
        discard;
    }

    return out;
}