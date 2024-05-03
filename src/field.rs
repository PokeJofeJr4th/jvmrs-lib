use std::{fmt::Display, sync::Arc};

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
        $crate::class::FieldType::Byte
    };
    (short) => {
        $crate::class::FieldType::Short
    };
    (int) => {
        $crate::class::FieldType::Int
    };
    (long) => {
        $crate::class::FieldType::Long
    };
    (float) => {
        $crate::class::FieldType::Float
    };
    (double) => {
        $crate::class::FieldType::Double
    };
    (char) => {
        $crate::class::FieldType::Char
    };
    (boolean) => {
        $crate::class::FieldType::Boolean
    };
    ([]$($rest:tt)*) => {
        $crate::class::FieldType::Array(Box::new($crate::field!($($rest)*)))
    };
    (Object($id:expr)) => {
        $crate::class::FieldType::Object($id)
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
