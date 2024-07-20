use chumsky::{error::Simple, Parser};
use lsystems::{LSystem, Value};

use super::Token;
use super::parser::parse_state;

pub struct LSystemBuilder {
    builder: lsystems::LSystemBuilder<Token>,
}

impl LSystemBuilder {
    pub fn new<S: AsRef<str>>(s: S) -> Result<LSystemBuilder, Vec<Simple<char>>> {
        Ok(LSystemBuilder {
            builder: lsystems::LSystemBuilder::new(parse_state().parse(s.as_ref())?),
        })
    }

    pub fn rule<S: AsRef<str>>(mut self, s: S) -> Result<LSystemBuilder, Vec<Simple<char>>> {
        self.builder = self
            .builder
            .rule(super::parser::parse_rule().parse(s.as_ref())?);
        Ok(self)
    }

    pub fn variable(mut self, key: char, value: Value) -> LSystemBuilder {
        self.builder = self.builder.variable(key, value);
        self
    }

    pub fn build(self) -> LSystem<Token> {
        self.builder.build()
    }
}
