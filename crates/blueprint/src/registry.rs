use bevy_utils::HashMap;
use derive_more::{Deref, DerefMut};

use crate::{
    graph::{Node, NodeConstructor},
    RegistrationError,
};

#[derive(Default, Deref, DerefMut)]
pub struct NodeRegistry(HashMap<String, Box<dyn Node>>);

impl NodeRegistry {
    pub fn init<'a, N: NodeConstructor<'a>>(&mut self) -> Result<(), RegistrationError> {
        if self.contains_key(N::NAME) {
            Err(RegistrationError::NotUnique)
        } else {
            self.insert(N::NAME.to_string(), Box::new(N::construct()));
            Ok(())
        }
    }

    pub fn register<'a, N: Node + 'static>(
        &mut self,
        named: &'a str,
        node: N,
    ) -> Result<(), RegistrationError> {
        if self.contains_key(named) {
            Err(RegistrationError::NotUnique)
        } else {
            self.insert(named.into(), Box::new(node));
            Ok(())
        }
    }

    pub fn remove<'a, N: NodeConstructor<'a>>(&mut self) {
        self.0.remove(N::NAME);
    }
}
