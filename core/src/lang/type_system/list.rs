use crate::{
    Error,
    lang::{
        invokable::operator,
        type_system::{InnerTypeTrait, TypeTrait},
    },
};

use super::Attribute;

#[derive(Debug, Clone, PartialEq)]
pub struct List<T: InnerTypeTrait> {
    data: Vec<T>,
    attributes: u8,
}

impl<T: InnerTypeTrait> List<T> {
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

macro_rules! impl_list_operator {
    ($trait_name:ident, $method_name:ident) => {
        impl<T: for<'a> operator::$trait_name<&'a T, T> + InnerTypeTrait>
            operator::$trait_name<&List<T>, List<T>> for &List<T>
        {
            fn $method_name(lhs: &List<T>, rhs: &List<T>) -> Result<List<T>, Error> {
                if lhs.count() != rhs.count() {
                    return Err(Error::Length);
                }
                let mut data = Vec::with_capacity(lhs.count());
                for (l, r) in lhs.data.iter().zip(rhs.data.iter()) {
                    data.push(T::$method_name(l, r)?);
                }
                Ok(List::new(data))
            }
        }
    };
}

impl_list_operator!(OperatorAdd, add);
impl_list_operator!(OperatorSubtract, sub);
impl_list_operator!(OperatorMultiply, mul);
impl_list_operator!(OperatorDivide, div);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::invokable::operator::OperatorAdd;
    use crate::lang::type_system::inner_types::TypeInt;

    #[test]
    fn test_add_typeint() {
        let list1 = List::new(vec![TypeInt::from(1), TypeInt::from(2), TypeInt::from(3)]);
        let list2 = List::new(vec![TypeInt::from(4), TypeInt::from(5), TypeInt::from(6)]);
        let result =
            <&List<TypeInt> as OperatorAdd<&List<TypeInt>, List<TypeInt>>>::add(&list1, &list2);

        assert!(result.is_ok());
        let result_list = result.unwrap();
        let expected_list = List::new(vec![TypeInt::from(5), TypeInt::from(7), TypeInt::from(9)]);
        assert_eq!(result_list, expected_list);
    }
}
