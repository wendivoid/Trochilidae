use crate::{Alphabet, Module, State};

pub struct Axiom<A: Alphabet> {
    pub(crate) inner: State<A>
}

impl <A: Alphabet>Axiom<A> {
    pub fn new(inner: State<A>) -> Axiom<A> {
        Axiom {
            inner
        }
    }
}

impl <A: Alphabet, I: Into<Module<A>>, Iter: IntoIterator<Item=I>>From<Iter> for Axiom<A> {
    fn from(inner: Iter) -> Axiom<A> {
        Axiom { inner: State::new(inner.into_iter().map(|x|x.into()).collect::<Vec<Module<A>>>()) }
    }
}