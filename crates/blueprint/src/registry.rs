use bevy_utils::HashMap;

use crate::{
    graph::{Node, NodeConstructor},
    RegistrationError,
};

#[derive(Default)]
pub struct NodeRegistry(HashMap<String, Box<dyn Node>>);

impl NodeRegistry {
    pub fn init<'a, N: NodeConstructor<'a>>(&mut self) -> Result<(), RegistrationError> {
        if self.0.contains_key(N::NAME) {
            Err(RegistrationError::NotUnique)
        } else {
            self.0.insert(N::NAME.to_string(), Box::new(N::construct()));
            Ok(())
        }
    }

    pub fn register<'a, N: Node + 'static>(
        &mut self,
        named: &'a str,
        node: N,
    ) -> Result<(), RegistrationError> {
        if self.0.contains_key(named) {
            Err(RegistrationError::NotUnique)
        } else {
            self.0.insert(named.into(), Box::new(node));
            Ok(())
        }
    }

    pub fn remove<'a, N: NodeConstructor<'a>>(&mut self) {
        self.0.remove(N::NAME);
    }

    pub fn get<'a>(&self, name: &'a str) -> Option<&Box<dyn Node>> {
        self.0.get(name)
    }

    pub fn get_mut<'a>(&mut self, name: &'a str) -> Option<&mut Box<dyn Node>> {
        self.0.get_mut(name)
    }
}
