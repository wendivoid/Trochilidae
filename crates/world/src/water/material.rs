use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::{prelude::*, ExtendedMaterial, MaterialExtension};
use bevy_reflect::TypePath;
use bevy_render::{
    render_asset::RenderAssets,
    render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    texture::GpuImage,
};

pub type StandardWaterMaterial = ExtendedMaterial<StandardMaterial, WaterMaterial>;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[uniform(100, WaterMaterialUniform)]
pub struct WaterMaterial {
    pub deep_color: LinearRgba,
    pub shallow_color: LinearRgba,
    pub edge_color: LinearRgba,
    pub clarity: f32,
    pub edge_scale: f32,
}

impl MaterialExtension for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/water.wgsl".into()
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct WaterMaterialUniform {
    pub deep_color: Vec4,
    pub shallow_color: Vec4,
    pub edge_color: Vec4,
    pub clarity: f32,
    pub edge_scale: f32,
}

impl AsBindGroupShaderType<WaterMaterialUniform> for WaterMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> WaterMaterialUniform {
        WaterMaterialUniform {
            deep_color: self.deep_color.to_vec4(),
            shallow_color: self.shallow_color.to_vec4(),
            edge_color: self.edge_color.to_vec4(),
            clarity: self.clarity,
            edge_scale: self.edge_scale,
        }
    }
}
