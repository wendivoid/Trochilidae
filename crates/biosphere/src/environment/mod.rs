mod cond;
mod op;
mod val;
mod var;

pub use self::cond::EnvCondition;
pub use self::op::EnvOperator;
pub use self::val::EnvValue;
pub use self::var::EnvVariable;

pub struct Environment {
    pub elevation: f32,
    pub moisture: f32,
    pub water_table: f32,
}