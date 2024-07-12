use bevy_math::prelude::*;
use bevy_color::prelude::*;
use bevy_render::prelude::*;

use bevy_transform::components::Transform;
use bevy_utils::HashMap;
use lsystems::{LSystem, Value};

use crate::builder::Token;
use super::data::MeshData;

use super::{MeshRenderConfig, MeshRenderState};

pub struct MeshRenderer<'a> {
    data: MeshData,
    lsys: &'a LSystem<Token>,
    cfg: MeshRenderConfig,
    state: MeshRenderState,
    polygon: Option<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 4]>, Vec<u32>)>,
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
        let mut locations = HashMap::new();

        let mut meshes = vec![];
        
        meshes.push(self.draw_segment(0.));
        
        for generation in 1..self.cfg.age {
            let mut stack = vec![];
            self.last_state = (0, (&self.cfg).into());
            self.state = (&self.cfg).into();
            let tokens = self.lsys.sample(generation as usize);
            for token in tokens {
                match &token.token {
                    Token::F => self.forward(&token, generation as f32, &mut locations, &mut meshes),
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
                    Token::External(o) => self.external(o),
                }
            }
        }
        let mut data: Mesh = meshes.remove(0);
        for m in meshes {
            data.merge(&m);
        }
        data
    }
    fn rotate(&mut self) {
        self.state
        .cursor
        .rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, 1.0, 0.0))
    }

    fn math<F: Fn(&mut Transform, f32)>(&mut self, token: &lsystems::Module<Token>, f: F) {
        self.state.angle(token.params.first());
        f(&mut self.state.cursor, self.state.angle.to_radians());
    }

    fn external(&mut self, o: &char) {
        if o.is_uppercase() {
            self.state.up();
        }
        if let Some(Value::Color(r, g, b, a)) = self.lsys.variables.get(o) {
            self.state.color = Srgba::new(*r, *g, *b, *a);
        }
    }

    fn forward(&mut self, token: &lsystems::Module<Token>, generation: f32, locations: &mut HashMap<u32, Vec3>, meshes: &mut Vec<Mesh>) {
        self.state.length(token.params.first());
        self.state.width(token.params.get(1));
        self.state.up();
        let contains_value = locations.iter().find(|(_, x)| *x == &self.state.cursor.translation);
        if self.polygon.is_none() && !contains_value.is_some() {
            let cursor = meshes.iter().map(|x|x.count_vertices()).sum::<usize>() as u32 + self.data.positions.len() as u32;
            locations.insert(cursor, self.state.cursor.translation);
            meshes.push(self.draw_segment(generation));
            self.last_state = (cursor, self.state.clone());
        } else if let Some((dex, _)) = contains_value {
            self.last_state = (*dex, self.state.clone());
        }
    }

    fn draw_segment(&mut self, generation: f32) -> Mesh {
        let mut data = MeshData::default();
        let last_state = self.last_state.1.clone();
        draw_circle(&self.cfg, &mut data, &last_state, &last_state, generation);
        draw_circle(&self.cfg, &mut data, &self.state, &last_state, generation);
        let ring = self.cfg.resolution;
        let last_ring = 0;

        for j in 0..self.cfg.resolution as u32 + 1 {
            data.indices.extend_from_slice(&[
                last_ring + j,
                ring + j,
                ring + j + 1,
                last_ring + j,
                ring + j + 1,
                last_ring + j + 1,
            ]);
        }
        data.into()
    }
}

fn draw_circle(cfg: &MeshRenderConfig, data: &mut MeshData, state: &MeshRenderState, other: &MeshRenderState, generation: f32) {
    let step_theta = std::f32::consts::TAU / cfg.resolution as f32;
    for segment in 0..=cfg.resolution {
        let theta = segment as f32 * step_theta;
        let (sin, cos) = theta.sin_cos();
        let radius = state.width / 2.0;
        let p = Vec3::new(radius * cos, 0.0, radius * sin);
        let mut transformer = state.cursor.clone();
        let angle = state.cursor.rotation.angle_between(other.cursor.rotation);
        transformer.rotate_axis(other.cursor.local_y().into(), -angle);
        data.positions
            .push(transformer.transform_point(p).to_array());
        data.colors.push(state.color.to_f32_array());
        data.generations.push(generation);
    }
}
