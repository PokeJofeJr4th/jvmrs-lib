use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitOr};

#[macro_export]
macro_rules! access {
    ($($acc:ident)*) => {
        $crate::AccessFlags(0) $(|$crate::access!(@$acc))*
    };

    (@public) => {
        $crate::AccessFlags::ACC_PUBLIC
    };

    (@private) => {
        $crate::AccessFlags::ACC_PRIVATE
    };

    (@protected) => {
        $crate::AccessFlags::ACC_PROTECTED
    };

    (@static) => {
        $crate::AccessFlags::ACC_STATIC
    };

    (@final) => {
        $crate::AccessFlags::ACC_FINAL
    };

    (@synchronized) => {
        $crate::AccessFlags::ACC_SYNCHRONIZED
    };

    (@volatile) => {
        $crate::AccessFlags::ACC_VOLATILE
    };

    (@transient) => {
        $crate::AccessFlags::ACC_TRANSIENT
    };

    (@native) => {
        $crate::AccessFlags::ACC_NATIVE
    };

    (@abstract) => {
        $crate::AccessFlags::ACC_ABSTRACT
    };

    (@strict) => {
        $crate::AccessFlags::ACC_STRICT
    };

    (@synthetic) => {
        $crate::AccessFlags::ACC_SYNTHETIC
    };

    (@enum) => {
        $crate::AccessFlags::ACC_ENUM
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// Flag Name           Value   Interpretation
/// `ACC_PUBLIC`          0x0001  Declared public; may be accessed from outside its package.
/// `ACC_PRIVATE`         0x0002  Declared private; usable only within the defining class.
/// `ACC_PROTECTED`       0x0004  Declared protected; may be accessed within subclasses.
/// `ACC_STATIC`          0x0008  Declared static.
/// `ACC_FINAL`           0x0010  Declared final; never directly assigned to after object construction (JLS §17.5).
/// `ACC_SYNCHRONIZED`    0x0020  Declared synchronized; invocation is wrapped by a monitor use.
/// `ACC_VOLATILE`        0x0040  Declared volatile; cannot be cached.
/// `ACC_TRANSIENT`       0x0080  Declared transient; not written or read by a persistent object manager.
/// `ACC_NATIVE`          0x0100  Declared native; implemented in a language other than Java.
/// `ACC_ABSTRACT`        0x0400  Declared abstract; no implementation is provided.
/// `ACC_STRICT`          0x0800  Declared strictfp; floating-point mode is FP-strict.
/// `ACC_SYNTHETIC`       0x1000  Declared synthetic; not present in the source code.
/// `ACC_ENUM`            0x4000  Declared as an element of an enum.
pub struct AccessFlags(pub u16);

impl Debug for AccessFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessFlags({self})")
    }
}

impl Display for AccessFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 & 1 != 0 {
            write!(f, "public ")?;
        }
        if self.0 & 2 != 0 {
            write!(f, "private ")?;
        }
        if self.0 & 4 != 0 {
            write!(f, "protected ")?;
        }
        if self.0 & 8 != 0 {
            write!(f, "static ")?;
        }
        if self.0 & 0x10 != 0 {
            write!(f, "final ")?;
        }
        if self.0 & 0x20 != 0 {
            write!(f, "synchronized ")?;
        }
        if self.0 & 0x40 != 0 {
            write!(f, "volatile ")?;
        }
        if self.0 & 0x80 != 0 {
            write!(f, "transient ")?;
        }
        if self.0 & 0x100 != 0 {
            write!(f, "native ")?;
        }
        if self.0 & 0x400 != 0 {
            write!(f, "abstract ")?;
        }
        if self.0 & 0x800 != 0 {
            write!(f, "fp-strict ")?;
        }
        if self.0 & 0x1000 != 0 {
            write!(f, "synthetic ")?;
        }
        if self.0 & 0x4000 != 0 {
            write!(f, "enum ")?;
        }
        Ok(())
    }
}

impl AccessFlags {
    #[must_use]
    pub const fn is_static(self) -> bool {
        self.0 & Self::ACC_STATIC.0 != 0
    }
    #[must_use]
    pub const fn is_native(self) -> bool {
        self.0 & Self::ACC_NATIVE.0 != 0
    }
    #[must_use]
    pub const fn is_abstract(self) -> bool {
        self.0 & Self::ACC_ABSTRACT.0 != 0
    }

    // pub const ZERO: Self = Self(0);
    pub const ACC_PUBLIC: Self = Self(0x0001);
    pub const ACC_PRIVATE: Self = Self(0x0002);
    pub const ACC_PROTECTED: Self = Self(0x0004);
    pub const ACC_STATIC: Self = Self(0x0008);
    pub const ACC_FINAL: Self = Self(0x0010);
    pub const ACC_SYNCHRONIZED: Self = Self(0x0020);
    pub const ACC_VOLATILE: Self = Self(0x0040);
    pub const ACC_TRANSIENT: Self = Self(0x0080);
    pub const ACC_NATIVE: Self = Self(0x0100);
    // pub const ACC_???: u16 = 0x0200;
    pub const ACC_ABSTRACT: Self = Self(0x0400);
    pub const ACC_STRICT: Self = Self(0x0800);
    pub const ACC_SYNTHETIC: Self = Self(0x1000);
    // pub const ACC_???: u16 = 0x2000;
    pub const ACC_ENUM: Self = Self(0x4000);
    // pub const ACC_???: u16 = 0x8000;
}

impl BitOr for AccessFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for AccessFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
