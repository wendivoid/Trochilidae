use bevy_asset::load_internal_asset;
use bevy_pbr::MaterialPlugin;
use bevy_app::prelude::*;
use bevy_render::render_resource::Shader;

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<crate::material::PlantMaterial>::default());
        load_internal_asset!(
            app,
            crate::PLANT_VERTEX_SHADER_HANDLE,
            "render/vertex_shader.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            crate::PLANT_FRAGMENT_SHADER_HANDLE,
            "render/fragment_shader.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            crate::PLANT_PREPASS_SHADER_HANDLE,
            "render/prepass_shader.wgsl",
            Shader::from_wgsl
        );
        
    }
}