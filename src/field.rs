use crate::masking::*;

/// Trait used to produce masked values
pub trait DeboogField<T = DisplayStr> {
    /// Used for `#[deboog(mask = "all")]`
    ///
    /// Default implementation is provided, which swaps the value for three asterisks.
    fn mask_all(&self) -> String {
        HIDE_STR.into()
    }

    /// Used for `#[deboog(mask = "pan")]`
    ///
    /// Default implementation is provided, which swaps the value for three asterisks.
    fn mask_pan(&self) -> String {
        HIDE_STR.into()
    }

    /// Used for `#[deboog(mask = "pan_suffix")]`
    ///
    /// Default implementation is provided, which swaps the value for three asterisks.
    fn mask_pan_suffix(&self) -> String {
        HIDE_STR.into()
    }

    /// Used for `#[deboog(mask = "hidden")]`
    ///
    /// Default implementation is provided, which swaps the value for three asterisks.
    fn mask_hide(&self) -> String {
        HIDE_STR.into()
    }
}

/// Marker type to trick compiler. Do not use.
#[doc(hidden)]
pub struct AsRefStr;
/// Marker type to trick compiler. Do not use.
#[doc(hidden)]
pub struct DisplayStr;

impl<T> DeboogField<AsRefStr> for T
where
    T: AsRef<str>,
{
    fn mask_all(&self) -> String {
        mask_all(self.as_ref())
    }

    fn mask_pan(&self) -> String {
        mask_pan(self.as_ref())
    }

    fn mask_pan_suffix(&self) -> String {
        mask_pan_suffix(self.as_ref())
    }
}

macro_rules! display_impl {
    ($t:ty) => {
        impl DeboogField for $t {
            fn mask_pan(&self) -> String {
                let plain = format!("{}", self);
                mask_pan(&plain)
            }

            fn mask_pan_suffix(&self) -> String {
                let plain = format!("{}", self);
                mask_pan_suffix(&plain)
            }

            fn mask_all(&self) -> String {
                let plain = format!("{}", self);
                mask_all(&plain)
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
