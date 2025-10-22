#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    MixedList(Vec<Value>),

    Bool(bool),
    Byte(u8),
    Char(char),
    Integer(i64),
    Double(f64),

    Bools(Vec<bool>),
    Bytes(Vec<u8>),
    Chars(Vec<char>),
    Integers(Vec<i64>),
    Doubles(Vec<f64>),

    Guid(uuid::Uuid),
    Guids(Vec<uuid::Uuid>),
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        for val in &v {
            match val {
                Value::MixedList(_) => {
                    return Value::MixedList(v);
                }
                _ => {}
            }
        }
        Value::MixedList(v)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<u8> for Value {
    fn from(b: u8) -> Self {
        Value::Byte(b)
    }
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        Value::Char(c)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Integer(i as i64)
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Self {
        Value::Integer(u as i64)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(i)
    }
}

impl From<f64> for Value {
    fn from(d: f64) -> Self {
        Value::Double(d)
    }
}

impl From<Vec<bool>> for Value {
    fn from(v: Vec<bool>) -> Self {
        Value::Bools(v)
    }
}
impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}
impl From<Vec<char>> for Value {
    fn from(v: Vec<char>) -> Self {
        Value::Chars(v)
    }
}

impl From<Vec<i64>> for Value {
    fn from(v: Vec<i64>) -> Self {
        Value::Integers(v)
    }
}

impl From<Vec<f64>> for Value {
    fn from(v: Vec<f64>) -> Self {
        Value::Doubles(v)
    }
}

impl From<uuid::Uuid> for Value {
    fn from(g: uuid::Uuid) -> Self {
        Value::Guid(g)
    }
}

impl From<Vec<uuid::Uuid>> for Value {
    fn from(v: Vec<uuid::Uuid>) -> Self {
        Value::Guids(v)
    }
}

impl Value {
    pub fn rank(&self) -> usize {
        match self {
            Value::MixedList(v) => v.len(),
            Value::Bools(v) => v.len(),
            Value::Bytes(v) => v.len(),
            Value::Chars(v) => v.len(),
            Value::Integers(v) => v.len(),
            Value::Doubles(v) => v.len(),
            _ => 1,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::MixedList(_) => "MixedList",
            Value::Bool(_) => "Bool",
            Value::Byte(_) => "Byte",
            Value::Char(_) => "Char",
            Value::Integer(_) => "Integer",
            Value::Double(_) => "Double",
            Value::Bools(_) => "Bools",
            Value::Bytes(_) => "Bytes",
            Value::Chars(_) => "Chars",
            Value::Integers(_) => "Integers",
            Value::Doubles(_) => "Doubles",
            Value::Guid(_) => "Guid",
            Value::Guids(_) => "Guids",
        }
    }

    pub fn full_type_name(&self) -> String {
        match self {
            Value::MixedList(l) => format!(
                "MixedList<{}>",
                l.iter()
                    .map(|v| v.full_type_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Bool(_) => "Bool".to_string(),
            Value::Byte(_) => "Byte".to_string(),
            Value::Char(_) => "Char".to_string(),
            Value::Integer(_) => "Integer".to_string(),
            Value::Double(_) => "Double".to_string(),
            Value::Bools(_) => "Bools".to_string(),
            Value::Bytes(_) => "Bytes".to_string(),
            Value::Chars(_) => "Chars".to_string(),
            Value::Integers(_) => "Integers".to_string(),
            Value::Doubles(_) => "Doubles".to_string(),
            Value::Guid(_) => "Guid".to_string(),
            Value::Guids(_) => "Guids".to_string(),
        }
    }

    pub fn full_type_name_with_rank(&self) -> String {
        match self {
            Value::MixedList(l) => format!(
                "MixedList[{}]<{}>",
                l.len(),
                l.iter()
                    .map(|v| v.full_type_name_with_rank())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Bool(_) => "Bool".to_string(),
            Value::Byte(_) => "Byte".to_string(),
            Value::Char(_) => "Char".to_string(),
            Value::Integer(_) => "Integer".to_string(),
            Value::Double(_) => "Double".to_string(),
            Value::Bools(_) => format!("Bools[{}]", self.rank()),
            Value::Bytes(_) => format!("Bytes[{}]", self.rank()),
            Value::Chars(_) => format!("Chars[{}]", self.rank()),
            Value::Integers(_) => format!("Integers[{}]", self.rank()),
            Value::Doubles(_) => format!("Doubles[{}]", self.rank()),
            Value::Guid(_) => "Guid".to_string(),
            Value::Guids(_) => format!("Guids[{}]", self.rank()),
        }
    }
}

impl IntoIterator for Value {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Value::MixedList(v) => v.into_iter(),
            Value::Bools(v) => v
                .iter()
                .map(|b| Value::Bool(*b))
                .collect::<Vec<_>>()
                .into_iter(),
            Value::Bytes(v) => v
                .iter()
                .map(|b| Value::Byte(*b))
                .collect::<Vec<_>>()
                .into_iter(),
            Value::Chars(v) => v
                .iter()
                .map(|c| Value::Char(*c))
                .collect::<Vec<_>>()
                .into_iter(),
            Value::Integers(v) => v
                .iter()
                .map(|i| Value::Integer(*i))
                .collect::<Vec<_>>()
                .into_iter(),
            Value::Doubles(v) => v
                .iter()
                .map(|d| Value::Double(*d))
                .collect::<Vec<_>>()
                .into_iter(),
            Value::Guids(v) => v
                .iter()
                .map(|g| Value::Guid(*g))
                .collect::<Vec<_>>()
                .into_iter(),
            scalar => vec![scalar.clone()].into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_values() {
        let v1 = Value::from(42);
        let v2 = Value::from(vec![1.0, 2.0, 3.0]);
        let v3 = Value::from(vec![v1.clone(), v2.clone()]);
        let v4: Value = 42.into();

        assert_eq!(v1, Value::Integer(42));
        assert_eq!(v2, Value::Doubles(vec![1.0, 2.0, 3.0]));
        assert_eq!(
            v3,
            Value::MixedList(vec![
                Value::Integer(42),
                Value::Doubles(vec![1.0, 2.0, 3.0])
            ])
        );
        assert_eq!(v4, Value::Integer(42));
    }

    #[test]
    fn full_type_name() {
        let v1 = Value::from(42);
        let v2 = Value::from(vec![1.0, 2.0, 3.0]);
        let v3 = Value::from(vec![v1.clone(), v2.clone()]);

        assert_eq!(v1.full_type_name(), "Integer");
        assert_eq!(v2.full_type_name(), "Doubles");
        assert_eq!(v3.full_type_name(), "MixedList<Integer, Doubles>");
    }

    #[test]
    fn full_type_name_with_rank() {
        let v1 = Value::from(42);
        let v2 = Value::from(vec![1.0, 2.0, 3.0]);
        let v3 = Value::from(vec![v1.clone(), v2.clone()]);

        assert_eq!(v1.full_type_name_with_rank(), "Integer");
        assert_eq!(v2.full_type_name_with_rank(), "Doubles[3]");
        assert_eq!(
            v3.full_type_name_with_rank(),
            "MixedList[2]<Integer, Doubles[3]>"
        );
    }
}
