use crate::{Alphabet, Module};

pub struct Context<'a, A: Alphabet> {
    pub previous: Option<&'a Module<A>>,
    pub next: Option<&'a Module<A>>
}