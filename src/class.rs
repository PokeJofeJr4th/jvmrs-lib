use std::fmt::Debug;
use std::sync::Arc;

use crate::{FieldType, MethodDescriptor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodHandle {
    GetField {
        class: Arc<str>,
        name: Arc<str>,
        field_type: FieldType,
    },
    GetStatic {
        class: Arc<str>,
        name: Arc<str>,
        field_type: FieldType,
    },
    PutField {
        class: Arc<str>,
        name: Arc<str>,
        field_type: FieldType,
    },
    PutStatic {
        class: Arc<str>,
        name: Arc<str>,
        field_type: FieldType,
    },
    InvokeVirtual {
        class: Arc<str>,
        name: Arc<str>,
        method_type: MethodDescriptor,
    },
    InvokeStatic {
        class: Arc<str>,
        name: Arc<str>,
        method_type: MethodDescriptor,
    },
    InvokeSpecial {
        class: Arc<str>,
        name: Arc<str>,
        method_type: MethodDescriptor,
    },
    NewInvokeSpecial {
        class: Arc<str>,
        name: Arc<str>,
        method_type: MethodDescriptor,
    },
    InvokeInterface {
        class: Arc<str>,
        name: Arc<str>,
        method_type: MethodDescriptor,
    },
}

/// A member of the constant pool
#[derive(Clone, PartialEq)]
pub enum Constant {
    String(Arc<str>),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    ClassRef(Arc<str>),
    StringRef(Arc<str>),
    FieldRef {
        class: Arc<str>,
        name: Arc<str>,
        field_type: FieldType,
    },
    MethodRef {
        class: Arc<str>,
        name: Arc<str>,
        method_type: MethodDescriptor,
    },
    InterfaceRef {
        class: Arc<str>,
        name: Arc<str>,
        interface_type: MethodDescriptor,
    },
    NameTypeDescriptor {
        name: Arc<str>,
        type_descriptor: Arc<str>,
    },
    MethodHandle(MethodHandle),
    MethodType(MethodDescriptor),
    // Dynamic {
    //     constant: u32,
    // },
    InvokeDynamic {
        bootstrap_index: u16,
        method_name: Arc<str>,
        method_type: MethodDescriptor,
    },
    // Module {
    //     identity: u16,
    // },
    // Package {
    //     identity: u16,
    // },
    Placeholder,
}

impl Constant {
    #[must_use]
    pub fn bytes(&self) -> Vec<u32> {
        match self {
            Self::Int(i) => vec![*i as u32],
            Self::Float(f) => vec![f.to_bits()],
            Self::Long(l) => vec![*l as u64 as u32, (*l as u64 >> 32) as u32],
            Self::Double(f) => {
                let bits = f.to_bits();
                vec![bits as u32, (bits >> 32) as u32]
            }
            _ => vec![0],
        }
    }
}

impl Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s:?}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::Long(l) => write!(f, "{l}"),
            Self::Double(d) => write!(f, "{d}"),
            Self::ClassRef(c) => write!(f, "class {c}"),
            Self::StringRef(s) => write!(f, "&{s:?}"),
            Self::FieldRef {
                class,
                name,
                field_type,
            } => write!(f, "Field({field_type} {class}.{name})"),
            Self::MethodRef {
                class,
                name,
                method_type,
            } => write!(f, "Method({method_type:?} {class}.{name})"),
            Self::InterfaceRef {
                class,
                name,
                interface_type,
            } => write!(f, "InterfaceMethod({interface_type:?} {class}.{name})"),
            Self::NameTypeDescriptor {
                name,
                type_descriptor,
            } => write!(f, "NameTypeDescriptor({type_descriptor} {name})"),
            Self::MethodHandle(handle) => write!(f, "MethodHandle({handle:?})"),
            Self::MethodType(method) => write!(f, "MethodType({method:?})"),
            Self::InvokeDynamic {
                bootstrap_index,
                method_name,
                method_type,
            } => write!(
                f,
                "InvokeDynamic(#{bootstrap_index} {method_type:?} {method_name})"
            ),
            Self::Placeholder => write!(f, "Placeholder"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ClassVersion {
    pub minor_version: u16,
    pub major_version: u16,
}
