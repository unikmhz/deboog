use crate::masking::*;

pub trait DeboogField<T> {
    fn mask_pan(&self) -> String;
    fn mask_pan_suffix(&self) -> String;
    fn mask_all(&self) -> String;

    fn mask_hide(&self) -> String {
        HIDE_STR.into()
    }
}

pub struct AsRefStr;
pub struct DisplayStr;

impl<T> DeboogField<AsRefStr> for T
where
    T: AsRef<str>,
{
    fn mask_pan(&self) -> String {
        mask_pan(self.as_ref())
    }

    fn mask_pan_suffix(&self) -> String {
        mask_pan_suffix(self.as_ref())
    }

    fn mask_all(&self) -> String {
        mask_all(self.as_ref())
    }
}

macro_rules! display_impl {
    ($t:ty) => {
        impl DeboogField<DisplayStr> for $t {
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
