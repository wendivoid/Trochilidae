use bevy_ecs::{prelude::*, query::QueryItem};
use bevy_math::Vec3;
use bevy_render::extract_component::ExtractComponent;
use bevy_utils::HashMap;
use derive_more::{Deref, DerefMut};

use bytemuck::{Pod, Zeroable};
use hexx::Hex;

#[derive(Clone, Debug, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct VascularData {
    pub position: Vec3,
    pub scale: f32,
    pub birthdate: f32,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct VascularInstanceMap(pub HashMap<Hex, VascularData>);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct VascularInstanceData(pub Vec<VascularData>);

impl ExtractComponent for VascularInstanceMap {
    type QueryData = &'static VascularInstanceMap;
    type QueryFilter = ();
    type Out = VascularInstanceData;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some(VascularInstanceData(item.values().cloned().collect()))
    }
}
