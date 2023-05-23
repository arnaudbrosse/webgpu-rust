use crate::core::attribute::Attribute;
use std::collections::HashMap;

pub struct Geometry {
    pub attributes: HashMap<String, Attribute>,
}

impl Geometry {
    pub fn new(attributes: HashMap<String, Attribute>) -> Self {
        Self { attributes }
    }
}
