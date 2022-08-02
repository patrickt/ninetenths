use std::rc::Rc;

use crate::dhall::raw;
use crate::core::*;

pub struct Vault {
    body_parts: Vec<Rc<raw::BodyPart>>,
}

impl <'a> Vault {
    pub fn get_body_part(&self, handle: Handle<raw::BodyPart>) -> &raw::BodyPart {
        &self.body_parts[handle.index]
    }
}
