mod rules;
mod value;
mod module;
mod context;
mod lsystem;
mod operator;
mod alphabet;

pub use self::value::Value;
pub use self::context::Context;
pub use self::module::Module;
pub use self::operator::Operator;
pub use self::alphabet::Alphabet;
pub use self::rules::{Rule, Rules, Conditional, Condition};
pub use self::lsystem::{Axiom, State, LSystem, LSystemBuilder};

pub type Parameters = Vec<Value>;
pub type Variables = bevy_utils::HashMap<char, Value>;

#[macro_export]
macro_rules! variables {
    () => {
        ::bevy_utils::HashMap::new()
    };

    ($($key:expr => $value:expr),+ $(,)?) => {
        ::bevy_utils::HashMap::from([ $(($key, $value)),* ])
    };
}