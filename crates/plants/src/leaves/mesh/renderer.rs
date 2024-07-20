use bevy_color::prelude::*;
use bevy_math::prelude::*;
use bevy_render::{mesh::{Indices, PrimitiveTopology}, prelude::*};

use bevy_transform::components::Transform;
use lsystems::{LSystem, Value};

use crate::builder::Token;

use super::{MeshRenderConfig, MeshRenderState};

pub struct MeshRenderer<'a> {
    lsys: &'a LSystem<Token>,
    cfg: MeshRenderConfig,
    state: MeshRenderState,
    last_state: (u32, MeshRenderState),
}

impl<'a> MeshRenderer<'a> {
    pub fn new(lsys: &'a LSystem<Token>, cfg: MeshRenderConfig) -> MeshRenderer {
        MeshRenderer {
            lsys,
            last_state: (0, (&cfg).into()),
            state: (&cfg).into(),
            cfg,
        }
    }

    pub fn build(mut self) -> Mesh {

        let mut polygons: Vec<Vec<Vec3>> = vec![];

        let mut stack = vec![];
        self.last_state = (0, (&self.cfg).into());
        self.state = (&self.cfg).into();
        let tokens = self.lsys.sample(self.cfg.generation as usize);
        for token in tokens {
            match &token.token {
                Token::F => {
                    self.forward(&token);
                }
                Token::Push => stack.push((self.last_state.clone(), self.state.clone())),
                Token::Pop => {
                    if let Some((lstate, s)) = stack.pop() {
                        self.last_state = lstate;
                        self.state = s;
                    }
                }
                Token::Rotate => self.rotate(),
                Token::Left => self.math(&token, |t, arg| t.rotate_local_z(arg)),
                Token::Right => self.math(&token, |t, arg| t.rotate_local_z(-arg)),
                Token::Down => self.math(&token, |t, arg| t.rotate_local_x(-arg)),
                Token::Up => self.math(&token, |t, arg| t.rotate_local_x(arg)),
                Token::Roll => self.math(&token, |t, arg| t.rotate_local_y(arg)),
                Token::CounterRoll => self.math(&token, |t, arg| t.rotate_local_y(-arg)),
                Token::External(o) => self.external(o),
                Token::StartPolygon => {
                    polygons.push(vec![]);
                },
                Token::PolygonVertex => {
                    polygons.last_mut().unwrap().push(self.state.cursor.translation);
                },
                Token::EndPolygon => {}
            }
        }

        let mut positions = vec![];
        let mut indices = vec![];
        for vertices in polygons.into_iter() {
            if vertices.len() == 3 {
                let cursor = positions.len() as u32;
                positions.extend(vertices.clone().into_iter());
                indices.extend([
                    cursor, cursor + 1, cursor + 2,
                    cursor, cursor + 2, cursor + 1
                ]);
            } else if vertices.len() == 4 {
                let cursor = positions.len() as u32;
                positions.extend(vertices.clone().into_iter());
                indices.extend([
                    cursor, cursor + 1, cursor + 2,
                    cursor, cursor + 2, cursor + 1,

                    cursor + 1, cursor + 2, cursor + 3,
                    cursor + 1, cursor + 3, cursor + 2
                ]);
            } else {
                let mut triangles: Vec<u32> = vec![];
                let mut earcut = earcut::Earcut::new();
                for _ in 0..500 {
                    earcut.earcut::<u32>(vertices.clone().into_iter().map(|x|[x.x, x.y]), &[], &mut triangles);
                }
                positions.extend(vertices);
                indices.extend(triangles);
            }
        }
        Mesh::new(PrimitiveTopology::TriangleList, Default::default())
           .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
           .with_inserted_indices(Indices::U32(indices))
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

    fn forward(
        &mut self,
        token: &lsystems::Module<Token>,
    ) {
        self.state.length(token.params.first());
        self.state.width(token.params.get(1));
        self.state.up();
    }
}