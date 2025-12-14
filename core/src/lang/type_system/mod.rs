mod atom;
mod inner_types;
mod list;
mod super_type;

pub use atom::Atom;
pub use inner_types::{
    TypeBool, TypeByte, TypeChar, TypeFloat, TypeInt, TypeMixedList, TypeOperator,
};
pub use list::List;
pub use super_type::SuperType;

use crate::Error;

#[repr(u8)]
#[rustfmt::skip] // to keep bits aligned
pub enum Attribute {
    None =    0b0000_0000,
    Sorted =  0b0000_0001,
    Unique =  0b0000_0010,
    Parted =  0b0000_0100,
    Grouped = 0b0000_1000,
}

impl Attribute {
    pub fn has_sorted(v: u8) -> bool {
        (v & Attribute::Sorted as u8) != 0
    }

    pub fn has_unique(v: u8) -> bool {
        (v & Attribute::Unique as u8) != 0
    }

    pub fn has_parted(v: u8) -> bool {
        (v & Attribute::Parted as u8) != 0
    }

    pub fn has_grouped(v: u8) -> bool {
        (v & Attribute::Grouped as u8) != 0
    }
}

pub trait TypeTrait {
    // Returns the type identifier.
    fn get_type() -> i16;

    // Returns the number of elements in this type.
    fn count(&self) -> usize;

    fn get_attributes(&self) -> u8;
    fn set_attribute(&mut self, attribute: Attribute) -> Result<(), Error>;
}

pub trait InnerTypeTrait {
    fn get_type() -> i16;
}
