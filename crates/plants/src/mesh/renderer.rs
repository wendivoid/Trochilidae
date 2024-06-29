use bevy_math::prelude::*;
use bevy_render::{
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use bevy_transform::components::Transform;
use lsystems::{LSystem, Value};

use crate::Token;

use super::{MeshRenderConfig, MeshRenderState};

#[derive(Default)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

impl Into<Mesh> for MeshData {
    fn into(self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, Default::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colors)
            .with_inserted_indices(Indices::U32(self.indices))
    }
}

pub struct MeshRenderer<'a> {
    data: MeshData,
    lsys: &'a LSystem<Token>,
    cfg: MeshRenderConfig,
    state: MeshRenderState,
    polygon: Option<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 4]>)>,
    last_state: (u32, MeshRenderState),
}

impl<'a> MeshRenderer<'a> {
    pub fn new(lsys: &'a LSystem<Token>, cfg: MeshRenderConfig) -> MeshRenderer {
        MeshRenderer {
            lsys,
            data: MeshData::default(),
            last_state: (0, (&cfg).into()),
            state: (&cfg).into(),
            cfg,
            polygon: None,
        }
    }

    pub fn build(mut self) -> Mesh {
        let tokens = self.lsys.sample(self.cfg.age);
        let mut stack = vec![];

        let mut out_meshes = vec![];

        self.draw_circle();

        for token in tokens {
            match &token.token {
                Token::F => self.forward(&token),
                Token::Push => stack.push((self.last_state.clone(), self.state.clone())),
                Token::Pop => {
                    if let Some((lstate, s)) = stack.pop() {
                        self.last_state = lstate;
                        self.state = s;
                    }
                }
                Token::Rotate => self.rotate(),
                Token::Left => self.math(&token, |t, arg|t.rotate_local_z(arg)),
                Token::Right => self.math(&token, |t, arg|t.rotate_local_z(-arg)),
                Token::Down => self.math(&token, |t, arg|t.rotate_local_x(-arg)),
                Token::Up => self.math(&token, |t, arg|t.rotate_local_x(arg)),
                Token::Roll => self.math(&token, |t, arg|t.rotate_local_y(arg)),
                Token::CounterRoll => self.math(&token, |t, arg|t.rotate_local_y(-arg)),
                Token::StartPolygon => self.start_polygon(),
                Token::StopPolygon => self.stop_polygon(&mut out_meshes),
                Token::PolygonVertex => self.polygon_vertex(),
                Token::External(o) => self.external(o),
            }
        }
        let mut mesh: Mesh = self.data.into();
        for m in out_meshes.into_iter() {
            mesh.merge(m)
        }

        mesh
    }
    fn rotate(&mut self) {
        self.state
        .cursor
        .rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, 1.0, 0.0))
    }

    fn start_polygon(&mut self) {
        self.polygon = Some((vec![], vec![], vec![]));
    }

    fn math<F: Fn(&mut Transform, f32)>(&mut self, token: &lsystems::Module<Token>, f: F) {
        self.state.angle(token.params.first());
        f(&mut self.state.cursor, self.state.angle.to_radians());
        //self.state.cursor.rotate_local_z(self.state.angle.to_radians())
    }

    fn stop_polygon(&mut self, out_meshes: &mut Vec<Mesh>) {
        if let Some((positions, normals, colors)) = self.polygon.take() {
            let vertices = positions
                .iter()
                .map(|x| [x[0], x[1]])
                .flatten()
                .collect::<Vec<f32>>();
            let indices = earcutr::earcut(&vertices, &[], 2)
                .unwrap()
                .into_iter()
                .map(|x| x as u32)
                .collect();
            out_meshes.push(
                Mesh::new(PrimitiveTopology::TriangleList, Default::default())
                    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
                    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
                    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
                    .with_inserted_indices(Indices::U32(indices)),
            );
        }
        self.polygon = None;
    }

    fn external(&mut self, o: &char) {
        if o.is_uppercase() {
            self.state.up();
        }
        if let Some(Value::Color(r, g, b, a)) = self.lsys.variables.get(o) {
            self.state.color = Color::rgba(*r, *g, *b, *a);
        }
    }

    fn polygon_vertex(&mut self) {
        if let Some((ref mut positions, ref mut normals, ref mut colors)) = self.polygon.as_mut() {
            positions.push(self.state.cursor.translation.to_array());
            normals.push([0.0, 1.0, 0.0]);
            colors.push(self.state.color.as_rgba_f32());
        }
    }

    fn forward(&mut self, token: &lsystems::Module<Token>) {
        self.state.length(token.params.first());
        self.state.width(token.params.get(1));
        self.state.up();
        if self.polygon.is_none() {
            let cursor = self.data.positions.len() as u32;
            self.draw_circle();
            let ring = cursor;
            let last_ring = self.last_state.0;

            for j in 0..self.cfg.resolution as u32 {
                self.data.indices.extend_from_slice(&[
                    ring + j,
                    ring + j + 1,
                    last_ring + j,
                    last_ring + j,
                    ring + j + 1,
                    last_ring + j + 1,
                ]);
            }
            self.last_state = (cursor, self.state.clone());
        }
    }

    fn draw_circle(&mut self) {
        let step_theta = std::f32::consts::TAU / self.cfg.resolution as f32;
        for segment in 0..=self.cfg.resolution {
            let theta = segment as f32 * step_theta;
            let (sin, cos) = theta.sin_cos();
            let p = Vec3::new(self.state.width / 2.0 * cos, 0.0, self.state.width / 2.0 * sin);
            self.data
                .positions
                .push(self.state.cursor.transform_point(p).to_array());
            self.data.normals.push([cos, 0., sin]);
            self.data.colors.push(self.state.color.as_rgba_f32());
        }
    }
}
