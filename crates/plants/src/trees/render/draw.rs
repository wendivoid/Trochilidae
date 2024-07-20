use bevy_ecs::{
    query::ROQueryItem,
    system::{
        lifetimeless::{Read, SRes},
        SystemParamItem,
    },
};
use bevy_pbr::{RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup};
use bevy_render::{
    mesh::{GpuBufferInfo, GpuMesh},
    render_asset::RenderAssets,
    render_phase::{
        PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
    },
};

use super::{IndexBindgroup, VascularBuffer};

pub type DrawVascular = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetInstanceIndexBindGroup<2>,
    DrawVascularInstanced,
);

pub struct DrawVascularInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawVascularInstanced {
    type Param = (SRes<RenderAssets<GpuMesh>>, SRes<RenderMeshInstances>);
    type ViewQuery = ();
    type ItemQuery = Read<VascularBuffer>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: Option<&'w VascularBuffer>,
        (meshes, render_mesh_instances): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(item.entity())
        else {
            return RenderCommandResult::Failure;
        };
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
            return RenderCommandResult::Failure;
        };
        let Some(instance_buffer) = instance_buffer else {
            return RenderCommandResult::Failure;
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed => {
                pass.draw(0..gpu_mesh.vertex_count, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}

pub struct SetInstanceIndexBindGroup<const N: usize>;

impl<P: PhaseItem, const N: usize> RenderCommand<P> for SetInstanceIndexBindGroup<N> {
    type Param = ();
    type ViewQuery = ();

    type ItemQuery = Read<IndexBindgroup>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: ROQueryItem<'w, Self::ViewQuery>,
        index_bindgroup: Option<&'w IndexBindgroup>,
        _: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        if let Some(index_bindgroup) = index_bindgroup {
            pass.set_bind_group(N, &index_bindgroup.bind_group, &[]);
            RenderCommandResult::Success
        } else {
            RenderCommandResult::Failure
        }
    }
}
