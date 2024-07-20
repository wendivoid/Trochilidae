use bevy_core_pipeline::{
    core_3d::Transparent3d,
    prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass},
};
use bevy_ecs::prelude::*;
use bevy_pbr::{MeshPipelineKey, RenderMeshInstances};
use bevy_render::{
    mesh::GpuMesh,
    render_asset::RenderAssets,
    render_phase::{DrawFunctions, PhaseItemExtraIndex, ViewSortedRenderPhases},
    render_resource::{PipelineCache, SpecializedMeshPipelines},
    view::{ExtractedView, Msaa},
};

use crate::vascular::render::{DrawVascular, VascularInstanceData, VascularPipeline};

#[allow(clippy::too_many_arguments)]
pub fn queue_vascular(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<VascularPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<VascularPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<GpuMesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    material_meshes: Query<Entity, With<VascularInstanceData>>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
    mut views: Query<(
        Entity,
        &ExtractedView,
        Has<DepthPrepass>,
        Has<NormalPrepass>,
        Has<MotionVectorPrepass>,
        Has<DeferredPrepass>,
    )>,
) {
    let draw_custom = transparent_3d_draw_functions.read().id::<DrawVascular>();

    let msaa_key =
        MeshPipelineKey::from_msaa_samples(msaa.samples()) | MeshPipelineKey::DEPTH_PREPASS;

    for (
        view_entity,
        view,
        _depth_prepass,
        _normal_prepass,
        _motion_vector_prepass,
        _deferred_prepass,
    ) in &mut views
    {
        let Some(transparent_phase) = transparent_render_phases.get_mut(&view_entity) else {
            continue;
        };

        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();
        for entity in &material_meshes {
            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(entity) else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let key =
                view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology());
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                .unwrap();
            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: draw_custom,
                distance: rangefinder.distance_translation(&mesh_instance.translation),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::NONE,
            });
        }
    }
}
