use std::{collections::HashMap, rc::Rc};

use crate::lang::SuperType;

#[derive(Clone, Debug, Default)]
pub struct Variables(HashMap<String, Rc<SuperType>>);

impl Variables {
    pub fn set_ref(&mut self, name: String, value: Rc<SuperType>) -> Rc<SuperType> {
        self.0.insert(name, value.clone());
        value
    }

    pub fn set_new(&mut self, name: String, value: SuperType) -> Rc<SuperType> {
        let rc_value = Rc::new(value);
        self.0.insert(name, rc_value.clone());
        rc_value
    }

    pub fn get(&self, name: &str) -> Option<Rc<SuperType>> {
        self.0.get(name).cloned()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
