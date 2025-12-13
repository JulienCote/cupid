use crate::{
    Error,
    type_system::{InnerTypeTrait, TypeTrait},
};

use super::Attribute;

#[derive(Debug, Clone, PartialEq)]
pub struct List<T> {
    data: Vec<T>,
    attributes: u8,
}

impl<T> List<T> {
    pub fn new(data: Vec<T>) -> Self {
        List {
            data,
            attributes: Attribute::None as u8,
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: InnerTypeTrait> TypeTrait for List<T> {
    fn count(&self) -> usize {
        self.data.len()
    }

    fn get_type() -> i16 {
        // list id is the absolute value of the inner type id
        T::get_type().abs()
    }

    fn get_attributes(&self) -> u8 {
        self.attributes
    }

    fn set_attribute(&mut self, attribute: Attribute) -> Result<(), Error> {
        // TODO: validate if the attribute can be applied
        self.attributes = attribute as u8;
        Ok(())
    }
}

impl<T: InnerTypeTrait> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
