use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
};

use crate::AccessFlags;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Object(Arc<str>),
    Short,
    Boolean,
    Array(Box<FieldType>),
}

#[macro_export]
macro_rules! field {
    (byte) => {
        $crate::FieldType::Byte
    };
    (short) => {
        $crate::FieldType::Short
    };
    (int) => {
        $crate::FieldType::Int
    };
    (long) => {
        $crate::FieldType::Long
    };
    (float) => {
        $crate::FieldType::Float
    };
    (double) => {
        $crate::FieldType::Double
    };
    (char) => {
        $crate::FieldType::Char
    };
    (boolean) => {
        $crate::FieldType::Boolean
    };
    ([]$($rest:tt)*) => {
        $crate::FieldType::Array(Box::new($crate::field!($($rest)*)))
    };
    (Object($id:expr)) => {
        $crate::FieldType::Object($id)
    };
    (($($t:tt)*)) => {
        $crate::field!($($t)*)
    }
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean => write!(f, "boolean"),
            Self::Byte => write!(f, "byte"),
            Self::Char => write!(f, "char"),
            Self::Double => write!(f, "double"),
            Self::Float => write!(f, "float"),
            Self::Int => write!(f, "int"),
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
            Self::Array(inner) => write!(f, "{inner}[]"),
            Self::Object(class) => write!(f, "{class}"),
        }
    }
}

impl FieldType {
    #[must_use]
    pub const fn get_size(&self) -> usize {
        match self {
            Self::Double | Self::Long => 2,
            _ => 1,
        }
    }

    #[must_use]
    pub const fn is_reference(&self) -> bool {
        matches!(self, Self::Array(_) | Self::Object(_))
    }

    #[must_use]
    pub const fn idx(&self) -> u64 {
        match self {
            Self::Byte => 0,
            Self::Char => 1,
            Self::Double => 2,
            Self::Float => 3,
            Self::Int => 4,
            Self::Long => 5,
            Self::Object(_) => 6,
            Self::Short => 7,
            Self::Boolean => 8,
            Self::Array(_) => 9,
        }
    }
}

pub struct FieldHandle<FT = FieldType> {
    pub ty: FT,
    pub name: Arc<str>,
    pub class: Arc<str>,
    pub access: AccessFlags,
}

impl<FT: Hash> Hash for FieldHandle<FT> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.name.hash(state);
        self.class.hash(state);
        self.access.hash(state);
    }
}

impl<FT: Display> Debug for FieldHandle<FT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}.{}",
            self.access, self.ty, self.class, self.name
        )
    }
}
