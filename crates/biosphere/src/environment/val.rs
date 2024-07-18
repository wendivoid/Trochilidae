use super::EnvVariable;

pub enum EnvValue {
    Val(f32),
    Var(EnvVariable),
}
