use chumsky::{Parser, error::Simple};
use lsystems::{LSystem, LSystemBuilder, Value};

use super::Token;

pub struct PlantBuilder {
    builder: LSystemBuilder<Token>
}

impl PlantBuilder {
    pub fn new<S: AsRef<str>>(s: S) -> Result<PlantBuilder, Vec<Simple<char>>> {
        Ok(PlantBuilder {
            builder: LSystemBuilder::new(super::parser::parse_state().parse(s.as_ref())?)
        })
    }

    pub fn rule<S: AsRef<str>>(mut self, s: S) -> Result<PlantBuilder, Vec<Simple<char>>> {
        self.builder = self.builder.rule(super::parser::parse_rule().parse(s.as_ref())?);
        Ok(self)
    }

    pub fn variable(mut self, key: char, value: Value) -> PlantBuilder {
        self.builder = self.builder.variable(key, value);
        self
    }

    pub fn build(self) -> LSystem<Token> {
        self.builder.build()
    }
}