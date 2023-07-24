use std::iter::repeat;

/// Default replacement value for `#[deboog(mask = "hidden")]`
pub const HIDE_STR: &str = "***";

/// Mask type
#[derive(Clone, Copy)]
pub enum MaskType {
    /// Replaces all characters with `*`
    All,
    /// Same as [`MaskType::All`], but leaves unmasked characters according to PAN masking convention
    ///
    /// Leaves 6 characters in the beginning and 4 characters at the end of the string.
    Pan,
    /// Replaces all but last four characters with a single `*` symbol
    PanSuffix,
}

/// Produces masked string based on mask type
pub fn mask(value: &str, mask_type: MaskType) -> String {
    match mask_type {
        MaskType::All => mask_all(value),
        MaskType::Pan => mask_pan(value),
        MaskType::PanSuffix => mask_pan_suffix(value),
    }
}

/// Replaces all characters with `*`
pub fn mask_all(value: &str) -> String {
    "*".repeat(value.len())
}

/// Same as [`mask_all`], but leaves unmasked characters according to PAN masking convention
///
/// Leaves 6 characters in the beginning and 4 characters at the end of the string.
pub fn mask_pan(value: &str) -> String {
    let sz = value.len();
    let limit = if sz > 4 { sz - 4 } else { 0 };
    value
        .chars()
        .enumerate()
        .map(|(i, c)| if i < 6 || i >= limit { c } else { '*' })
        .collect()
}

/// Replaces all but last four characters with a single `*` symbol
pub fn mask_pan_suffix(value: &str) -> String {
    let sz = value.len();
    let limit = if sz > 4 { sz - 4 } else { 0 };
    repeat('*')
        .take(if sz > 4 { 1 } else { 0 })
        .chain(value[limit..].chars())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_all() {
        assert_eq!(mask_all(""), "");
        assert_eq!(mask_all("0"), "*");
        assert_eq!(mask_all("0123456789"), "**********");
    }

    #[test]
    fn test_mask_pan() {
        assert_eq!(mask_pan(""), "");
        assert_eq!(mask_pan("0"), "0");
        assert_eq!(mask_pan("01"), "01");
        assert_eq!(mask_pan("012345"), "012345");
        assert_eq!(mask_pan("0123456"), "0123456");
        assert_eq!(mask_pan("0123456789"), "0123456789");
        assert_eq!(mask_pan("01234567890"), "012345*7890");
        assert_eq!(mask_pan("0123456789012345"), "012345******2345");
    }

    #[test]
    fn test_mask_pan_suffix() {
        assert_eq!(mask_pan_suffix(""), "");
        assert_eq!(mask_pan_suffix("0"), "0");
        assert_eq!(mask_pan_suffix("01"), "01");
        assert_eq!(mask_pan_suffix("0123"), "0123");
        assert_eq!(mask_pan_suffix("01234"), "*1234");
        assert_eq!(mask_pan_suffix("012345"), "*2345");
        assert_eq!(mask_pan_suffix("0123456"), "*3456");
        assert_eq!(mask_pan_suffix("0123456789"), "*6789");
        assert_eq!(mask_pan_suffix("01234567890"), "*7890");
        assert_eq!(mask_pan_suffix("0123456789012345"), "*2345");
    }
}
