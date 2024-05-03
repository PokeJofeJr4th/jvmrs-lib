use std::fmt::Debug;
use std::hash::Hash;

use crate::field::FieldType;

#[derive(Clone, PartialEq, Eq)]
pub struct MethodDescriptor {
    pub parameter_size: usize,
    /// list of parameter types
    pub parameters: Vec<FieldType>,
    /// method return type; None => void
    pub return_type: Option<FieldType>,
}

impl MethodDescriptor {
    pub const EMPTY: Self = Self {
        parameter_size: 0,
        parameters: Vec::new(),
        return_type: None,
    };
}

#[macro_export]
macro_rules! method {
    (($($params:tt),*) -> void) => {{
        let parameters: Vec<$crate::FieldType> = vec![$($crate::field!($params)),*];
        $crate::MethodDescriptor {
            parameter_size: parameters.iter().map(|param| param.get_size()).sum(),
            parameters,
            return_type: None,
        }
    }};

    (($($params:tt),*) -> $($out:tt)*) => {{
        let parameters: Vec<$crate::FieldType> = vec![$($crate::field!($params)),*];
        $crate::MethodDescriptor {
            parameter_size: parameters.iter().map(|param| param.get_size()).sum(),
            parameters,
            return_type: Some($crate::field!($($out)*)),
        }
    }};
}

impl Hash for MethodDescriptor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parameters.hash(state);
        self.return_type.hash(state);
    }
}

impl Debug for MethodDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.return_type {
            Some(t) => {
                write!(f, "{t} ")?;
            }
            None => {
                write!(f, "void ")?;
            }
        }
        write!(
            f,
            "{}({})",
            self.parameter_size,
            self.parameters
                .iter()
                .map(|par| format!("{par}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
