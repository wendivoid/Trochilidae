use super::{EnvOperator, EnvValue, EnvVariable};

pub struct EnvCondition {
    pub var: EnvVariable,
    pub op: EnvOperator,
    pub val: EnvValue,
}
