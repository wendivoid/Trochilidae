use bevy_math::prelude::*;
use bevy_render::{
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use curvo::prelude::{AdaptiveTessellationOptions, NurbsCurve3D, NurbsSurface};
use lsystems::{LSystem, Value};
use nalgebra::{Point3, Vector3};

use crate::Token;

use super::{MeshRenderConfig, MeshRenderState};

pub struct MeshRenderer<'a> {
    lsys: &'a LSystem<Token>,
    cfg: MeshRenderConfig,
}

impl<'a> MeshRenderer<'a> {
    pub fn new(lsys: &'a LSystem<Token>, cfg: MeshRenderConfig) -> MeshRenderer {
        MeshRenderer { lsys, cfg }
    }

    pub fn build(&self) -> Mesh {
        let tokens = self.lsys.sample(self.cfg.generation);
        let mut stack = vec![];
        let mut state: MeshRenderState = (&self.cfg).into();

        let mut out_meshes = vec![];

        let mut last_state = state.clone();

        let mut polygon: Option<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 4]>)> = None;
        for token in tokens {
            match &token.token {
                Token::F => {
                    state.length(token.params.first());
                    state.width(token.params.get(1));
                    state.up();
                    if polygon.is_none() {
                        let segment = nurbs_surface(&last_state, &mut state).unwrap();
                        out_meshes.push(segment);
                    }
                    last_state = state.clone();
                }
                Token::Push => stack.push((last_state.clone(), state.clone())),
                Token::Pop => {
                    if let Some((lstate, s)) = stack.pop() {
                        last_state = lstate;
                        state = s;
                    }
                }
                Token::Rotate => {
                    state
                        .cursor
                        .rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, 1.0, 0.0))
                }
                Token::Left => {
                    state.angle(token.params.first());
                    state.cursor.rotate_local_z(state.angle.to_radians())
                }
                Token::Right => {
                    state.angle(token.params.first());
                    state.cursor.rotate_local_z(-state.angle.to_radians())
                }
                Token::Up => {
                    state.angle(token.params.first());
                    state.cursor.rotate_local_x(state.angle.to_radians())
                }
                Token::Down => {
                    state.angle(token.params.first());
                    state.cursor.rotate_local_x(-state.angle.to_radians())
                }
                Token::Roll => {
                    state.angle(token.params.first());
                    state.cursor.rotate_local_y(state.angle.to_radians())
                }
                Token::CounterRoll => {
                    state.angle(token.params.first());
                    state.cursor.rotate_local_y(-state.angle.to_radians())
                }
                Token::StartPolygon => {
                    polygon = Some((vec![], vec![], vec![]));
                }
                Token::StopPolygon => {
                    if let Some((positions, normals, colors)) = polygon {
                        let mut indices = vec![];
                        for (dex, _) in positions
                            .iter()
                            .enumerate()
                            .skip(1)
                            .take(positions.len() - 2)
                        {
                            indices.extend([
                                dex as u32 - 1,
                                dex as u32,
                                dex as u32 + 1,
                                dex as u32 - 1,
                                dex as u32 + 1,
                                dex as u32,
                            ]);
                        }
                        println!("{indices:?}");
                        out_meshes.push(
                            Mesh::new(PrimitiveTopology::TriangleList, Default::default())
                                .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
                                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
                                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
                                .with_inserted_indices(Indices::U32(indices)),
                        );
                    }
                    polygon = None;
                }
                Token::PolygonVertex => {
                    if let Some((ref mut positions, ref mut normals, ref mut colors)) =
                        polygon.as_mut()
                    {
                        positions.push(state.cursor.translation.to_array());
                        normals.push([0.0, 1.0, 0.0]);
                        colors.push(state.color.as_rgba_f32());
                    }
                }
                Token::External(o) => {
                    if o.is_uppercase() {
                        state.up();
                    }
                    if let Some(Value::Color(r, g, b, a)) = self.lsys.variables.get(o) {
                        state.color = Color::rgba(*r, *g, *b, *a);
                    }
                }
            }
        }

        let mut mesh: Mesh = out_meshes.remove(0);

        for m in out_meshes.into_iter() {
            mesh.merge(m)
        }

        mesh
    }
}

fn tess_options() -> AdaptiveTessellationOptions<f32> {
    AdaptiveTessellationOptions {
        min_depth: 0,
        max_depth: 0,
        ..Default::default()
    }
}

fn nurbs_surface(
    last_state: &MeshRenderState,
    state: &mut MeshRenderState,
) -> Result<Mesh, anyhow::Error> {
    let old_pos = last_state.cursor.translation;
    let x_axis = state.cursor.local_x().into();
    let z_axis = state.cursor.local_z().into();
    let pos = state.cursor.translation;
    let start = nurbs_circle(old_pos, x_axis, z_axis, last_state.width)?;
    let end = nurbs_circle(pos, x_axis, z_axis, state.width)?;
    let surface = NurbsSurface::try_loft(&[start, end], None)?;
    let tess = surface.tessellate(Some(tess_options())).cast::<f32>();
    let vertices: Vec<[f32; 3]> = tess.points().iter().map(|pt| (*pt).into()).collect();
    let normals: Vec<[f32; 3]> = tess.normals().iter().map(|n| (*n).into()).collect();
    let indices: Vec<u32> = tess
        .faces()
        .iter()
        .flat_map(|f| f.iter().map(|i| *i as u32))
        .collect();
    Ok(
        Mesh::new(PrimitiveTopology::TriangleList, Default::default())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_COLOR,
                vertices
                    .iter()
                    .map(|_| state.color.as_rgba_f32())
                    .collect::<Vec<[f32; 4]>>(),
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_indices(bevy_render::mesh::Indices::U32(indices)),
    )
}

fn nurbs_circle(
    pos: Vec3,
    x_axis: Vec3,
    z_axis: Vec3,
    width: f32,
) -> Result<NurbsCurve3D<f32>, anyhow::Error> {
    Ok(NurbsCurve3D::try_circle(
        &Point3::new(pos.x, pos.y, pos.z),
        &Vector3::new(x_axis.x, x_axis.y, x_axis.z),
        &Vector3::new(z_axis.x, z_axis.y, z_axis.z),
        width / 2.0,
    )?)
}
