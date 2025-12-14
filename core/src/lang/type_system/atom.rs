use super::Attribute;
use crate::{
    Error,
    lang::type_system::{InnerTypeTrait, List, TypeTrait},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Atom<T> {
    data: T,
}

impl<T> Atom<T> {
    pub fn new(data: T) -> Self {
        Atom { data }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn enlist(self) -> List<T> {
        List::new(vec![self.data])
    }
}

impl<T: InnerTypeTrait> TypeTrait for Atom<T> {
    fn count(&self) -> usize {
        1
    }

    fn get_type() -> i16 {
        T::get_type()
    }

    fn get_attributes(&self) -> u8 {
        Attribute::None as u8
    }

    fn set_attribute(&mut self, _attribute: Attribute) -> Result<(), Error> {
        Err(Error::Type)
    }
}
