use bevy_color::prelude::*;

#[derive(Clone)]
pub struct MeshRenderConfig {
    pub angle: f32,
    pub width: f32,
    pub length: f32,
    pub color: Srgba,
    pub resolution: u32,
    pub age: u32,
}

impl Default for MeshRenderConfig {
    fn default() -> MeshRenderConfig {
        MeshRenderConfig {
            angle: 16.0,
            width: 0.5,
            length: 1.0,
            age: 10,
            resolution: 6,
            color: Srgba::hex("#cca94a00").unwrap(),
        }
    }
}

impl<'a> Into<super::MeshRenderState> for &'a MeshRenderConfig {
    fn into(self) -> super::MeshRenderState {
        let MeshRenderConfig {
            length,
            width,
            angle,
            color,
            ..
        } = self;
        super::MeshRenderState {
            length: *length,
            width: *width,
            angle: *angle,
            color: color.clone(),
            cursor: Default::default(),
        }
    }
}
