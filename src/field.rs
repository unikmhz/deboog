use std::fmt::{self, Debug, Formatter};

use crate::masking::*;

/// Wrapper object to specify type of masking
pub enum Masked<'a, T: DeboogField> {
    /// Replaces all characters with `*`
    All(&'a T),
    /// Same as [`Masked::All`], but leaves unmasked characters according to PAN masking convention
    ///
    /// Leaves 6 characters in the beginning and 4 characters at the end of the string.
    Pan(&'a T),
    /// Replaces all but last four characters with a single `*` symbol
    PanSuffix(&'a T),
    /// Replaces all characters with a fixed string
    Hidden(&'a T),
}

impl<'a, T: DeboogField> Debug for Masked<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Masked::All(value) => value.fmt_masked(f, MaskType::All),
            Masked::Pan(value) => value.fmt_masked(f, MaskType::Pan),
            Masked::PanSuffix(value) => value.fmt_masked(f, MaskType::PanSuffix),
            Masked::Hidden(_) => write!(f, "{}", HIDE_STR),
        }
    }
}

/// Trait used to produce masked values
pub trait DeboogField {
    /// Writes masked debug value
    #[allow(unused_variables)]
    fn fmt_masked(&self, f: &mut std::fmt::Formatter<'_>, mask_type: MaskType) -> std::fmt::Result {
        write!(f, "{}", HIDE_STR)
    }
}

impl DeboogField for String {
    fn fmt_masked(&self, f: &mut std::fmt::Formatter<'_>, mask_type: MaskType) -> std::fmt::Result {
        let masked = mask(self, mask_type);
        write!(f, "\"{}\"", masked)
    }
}

impl DeboogField for &str {
    fn fmt_masked(&self, f: &mut std::fmt::Formatter<'_>, mask_type: MaskType) -> std::fmt::Result {
        let masked = mask(self, mask_type);
        write!(f, "\"{}\"", masked)
    }
}

impl<T: DeboogField> DeboogField for &T {
    fn fmt_masked(&self, f: &mut std::fmt::Formatter<'_>, mask_type: MaskType) -> std::fmt::Result {
        (*self).fmt_masked(f, mask_type)
    }
}

impl<T: DeboogField> DeboogField for Option<T> {
    fn fmt_masked(&self, f: &mut std::fmt::Formatter<'_>, mask_type: MaskType) -> std::fmt::Result {
        match self {
            Some(ref value) => {
                write!(f, "Some(")?;
                value.fmt_masked(f, mask_type)?;
                write!(f, ")")
            }
            None => write!(f, "None"),
        }
    }
}

impl<T: DeboogField> DeboogField for Vec<T> {
    fn fmt_masked(&self, f: &mut std::fmt::Formatter<'_>, mask_type: MaskType) -> std::fmt::Result {
        let mut first = true;
        write!(f, "[")?;
        for item in self {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            item.fmt_masked(f, mask_type)?;
        }
        write!(f, "]")
    }
}

macro_rules! display_impl {
    ($t:ty) => {
        impl DeboogField for $t {
            fn fmt_masked(
                &self,
                f: &mut std::fmt::Formatter<'_>,
                mask_type: MaskType,
            ) -> std::fmt::Result {
                let plain = format!("{}", self);
                write!(f, "{}", mask(&plain, mask_type))
            }
        }
    };
}

display_impl!(bool);

display_impl!(i8);
display_impl!(i16);
display_impl!(i32);
display_impl!(i64);
display_impl!(i128);
display_impl!(isize);

display_impl!(u8);
display_impl!(u16);
display_impl!(u32);
display_impl!(u64);
display_impl!(u128);
display_impl!(usize);

display_impl!(f32);
display_impl!(f64);

display_impl!(std::num::NonZeroI8);
display_impl!(std::num::NonZeroI16);
display_impl!(std::num::NonZeroI32);
display_impl!(std::num::NonZeroI64);
display_impl!(std::num::NonZeroI128);
display_impl!(std::num::NonZeroIsize);

display_impl!(std::num::NonZeroU8);
display_impl!(std::num::NonZeroU16);
display_impl!(std::num::NonZeroU32);
display_impl!(std::num::NonZeroU64);
display_impl!(std::num::NonZeroU128);
display_impl!(std::num::NonZeroUsize);
