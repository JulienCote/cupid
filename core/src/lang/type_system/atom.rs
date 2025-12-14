use super::Attribute;
use crate::{
    Error,
    lang::{
        invokable::operator,
        type_system::{InnerTypeTrait, List, TypeTrait},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Atom<T: InnerTypeTrait> {
    data: T,
}

impl<T: InnerTypeTrait> Atom<T> {
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

macro_rules! impl_atom_operator {
    ($trait_name:ident, $method_name:ident) => {
        impl<T: for<'a> operator::$trait_name<&'a T, T> + InnerTypeTrait>
            operator::$trait_name<&Atom<T>, Atom<T>> for &Atom<T>
        {
            fn $method_name(lhs: &Atom<T>, rhs: &Atom<T>) -> Result<Atom<T>, Error> {
                Ok(Atom::new(T::$method_name(lhs.data(), rhs.data())?))
            }
        }
    };
}

impl_atom_operator!(OperatorAdd, add);
impl_atom_operator!(OperatorSubtract, sub);
impl_atom_operator!(OperatorMultiply, mul);
impl_atom_operator!(OperatorDivide, div);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::invokable::operator::OperatorAdd;
    use crate::lang::type_system::inner_types::TypeInt;

    #[test]
    fn test_add_typeint() {
        let atom1 = Atom::new(TypeInt::from(5));
        let atom2 = Atom::new(TypeInt::from(10));

        let result =
            <&Atom<TypeInt> as OperatorAdd<&Atom<TypeInt>, Atom<TypeInt>>>::add(&atom1, &atom2);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().data().0, 15);
    }

    // #[test]
    // fn test_add_typefloat() {
    //     let atom1 = Atom::new(crate::lang::type_system::inner_types::TypeFloat::from(5.5));
    //     let atom2 = Atom::new(crate::lang::type_system::inner_types::TypeFloat::from(10.2));

    //     let result = <&Atom<crate::lang::type_system::inner_types::TypeFloat> as OperatorAdd<
    //         &Atom<crate::lang::type_system::inner_types::TypeFloat>,
    //         Atom<crate::lang::type_system::inner_types::TypeFloat>,
    //     >>::add(&atom1, &atom2);

    //     assert_eq!(result.data().0, 15.7);
    // }
}
