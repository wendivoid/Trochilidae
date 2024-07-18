#import bevy_pbr::{
	forward_io::{VertexOutput, FragmentOutput},
	pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_fragment::pbr_input_from_standard_material,
    view_transformations::depth_ndc_to_view_z,
}

struct WaterMaterial {
  deep_color: vec4<f32>,
  shallow_color: vec4<f32>,
  edge_color: vec4<f32>,
  clarity: f32,
  edge_scale: f32
};

@group(2) @binding(100)
var<uniform> water_material: WaterMaterial;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> FragmentOutput {
    let water_clarity = water_material.clarity;
    let deep_color = water_material.deep_color;
    let shallow_color = water_material.shallow_color;
    let edge_scale = water_material.edge_scale;
    let edge_color = water_material.edge_color;

    let z_depth_buffer_ndc = bevy_pbr::prepass_utils::prepass_depth(mesh.position, 0u);
    let z_depth_buffer_view = depth_ndc_to_view_z(z_depth_buffer_ndc);
    let z_fragment_view = depth_ndc_to_view_z(mesh.position.z);
    let depth_diff_view = z_fragment_view - z_depth_buffer_view;
    let beers_law = exp(-depth_diff_view * water_clarity);
    let depth_color = vec4<f32>(mix(deep_color.xyz, shallow_color.xyz, beers_law), 1.0 - beers_law);
    let water_color = mix(edge_color, depth_color, smoothstep(0.0, edge_scale, depth_diff_view));

    //var out: FragmentOutput;
    //out.color = water_color;
    //return water_color;
    var pbr_input = pbr_input_from_standard_material(mesh, true);
    pbr_input.material.base_color *= water_color;
    //pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}