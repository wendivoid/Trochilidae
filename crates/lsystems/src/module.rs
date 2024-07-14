use std::fmt;

use crate::{Alphabet, Parameters};

#[derive(Debug, PartialEq, Clone)]
pub struct Module<A: Alphabet> {
    pub token: A,
    pub params: Parameters,
}

impl<A: Alphabet> Module<A> {
    pub fn new(token: A) -> Module<A> {
        Module {
            token,
            params: Default::default(),
        }
    }

    pub fn params(mut self, params: Parameters) -> Self {
        self.params = params;
        self
    }
}

impl<A: Alphabet> From<A> for Module<A> {
    fn from(token: A) -> Self {
        Module {
            token,
            params: Default::default(),
        }
    }
}

impl<A: Alphabet + fmt::Display> fmt::Display for Module<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}
