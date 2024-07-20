use bevy_app::prelude::*;
use bevy_core_pipeline::core_3d::Transparent3d;
use bevy_ecs::prelude::*;
use bevy_pbr::MeshPipeline;
use bevy_render::batching::gpu_preprocessing::batch_and_prepare_sorted_render_phase;
use bevy_render::render_phase::AddRenderCommand;
use bevy_render::RenderSet;
use bevy_render::{
    extract_component::ExtractComponentPlugin, render_resource::SpecializedMeshPipelines, Render,
    RenderApp,
};

use super::data::VascularInstanceMap;
use super::{systems, DrawVascular, VascularPipeline};

pub struct VascularMaterialPlugin {
    auto_extract: bool
}

impl Default for VascularMaterialPlugin {
    fn default() -> Self {
        Self {
            auto_extract: true
        }
    }
}

impl VascularMaterialPlugin {
    pub fn dont_extract() -> VascularMaterialPlugin {
        Self { auto_extract: false }
    }
}

impl Plugin for VascularMaterialPlugin {
    fn build(&self, app: &mut App) {
        if self.auto_extract {
            app.add_plugins(ExtractComponentPlugin::<VascularInstanceMap>::default());
        }
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawVascular>()
            .init_resource::<SpecializedMeshPipelines<VascularPipeline>>()
            .add_systems(
                Render,
                (
                    systems::queue_vascular.in_set(RenderSet::QueueMeshes),
                    (
                        systems::prepare_instance_buffers,
                        systems::prepare_instance_index.after(
                            batch_and_prepare_sorted_render_phase::<Transparent3d, MeshPipeline>,
                        ),
                    )
                        .in_set(RenderSet::PrepareResources),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<VascularPipeline>();
    }
}
