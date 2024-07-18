use crate::environment::EnvCondition;

pub struct Habitability {
    pub spawn: Vec<EnvCondition>,
    pub daily: Vec<EnvCondition>,
    pub yearly: Vec<EnvCondition>,
}
