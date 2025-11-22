use crate::type_system::Type;

/// SuperType is a wrapper that provides a unified interface for all types
/// implementing the Type trait. It allows for type-safe operations while
/// maintaining the state type pattern.
#[derive(Clone, Debug)]
pub struct SuperType<T: Type>(pub T);

impl<T: Type> Type for SuperType<T> {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn type_id(&self) -> char {
        self.0.type_id()
    }

    fn attributes(&self) -> u8 {
        self.0.attributes()
    }

    fn size(&self) -> usize {
        self.0.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_system::inner_types::TypeInt;

    #[test]
    fn super_type() {
        let int_type = SuperType(TypeInt(42));
        assert_eq!(int_type.name(), "Int");
        assert_eq!(int_type.type_id(), 'i');
        assert_eq!(int_type.attributes(), 0);
        assert_eq!(int_type.size(), 1);
    }
}
