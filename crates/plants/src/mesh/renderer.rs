use bevy_math::prelude::*;
use bevy_render::{
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use lsystems::{LSystem, Value};

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
        let tokens = self.lsys.sample(self.cfg.age);
        let mut stack = vec![];
        let mut state: MeshRenderState = (&self.cfg).into();

        let mut out_meshes = vec![];

        let mut last_state = (0, state.clone());

        let mut positions = vec![];
        let mut normals = vec![];
        let mut indices = vec![];
        self.draw_circle(&mut positions, &mut normals, &state, self.cfg.resolution);

        let mut polygon: Option<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 4]>)> = None;
        for token in tokens {
            match &token.token {
                Token::F => {
                    state.length(token.params.first());
                    state.width(token.params.get(1));
                    state.up();
                    if polygon.is_none() {
                        let cursor = positions.len() as u32;
                        self.draw_circle(&mut positions, &mut normals, &state, self.cfg.resolution);
                        let ring = cursor;
                        let last_ring = last_state.0;
            
                        for j in 0..self.cfg.resolution as u32 {
                            indices.extend_from_slice(&[
                                ring + j,
                                ring + j + 1,
                                last_ring + j,
                                last_ring + j,
                                ring + j + 1,
                                last_ring + j + 1,
                            ]);
                        }
                        last_state = (cursor, state.clone());
                    }
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
                        let vertices = positions.iter().map(|x|[x[0], x[1]]).flatten().collect::<Vec<f32>>();
                        let indices = earcutr::earcut(&vertices, &[],2).unwrap().into_iter().map(|x|x as u32).collect();
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
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_indices(Indices::U32(indices));
        for m in out_meshes.into_iter() {
            mesh.merge(m)
        }

        mesh
    }

    fn draw_circle(&self, positions: &mut Vec<[f32 ; 3]>, normals: &mut Vec<[f32 ; 3]>, state: &MeshRenderState, resolution: u32) {
        let step_theta = std::f32::consts::TAU / resolution as f32;
        for segment in 0..=resolution {
            let theta = segment as f32 * step_theta;
            let (sin, cos) = theta.sin_cos();
            let p = Vec3::new(
                state.width / 2.0 * cos,
                0.0,
                state.width / 2.0 * sin
            );
            positions.push(state.cursor.transform_point(p).to_array());
            normals.push([cos, 0., sin]);
        }
    }
}