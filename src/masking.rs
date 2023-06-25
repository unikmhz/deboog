use std::iter::repeat;

pub const HIDE_STR: &str = "***";

pub fn mask_all(value: &str) -> String {
    "*".repeat(value.len())
}

pub fn mask_pan(value: &str) -> String {
    let sz = value.len();
    let limit = if sz > 4 { sz - 4 } else { 0 };
    value
        .chars()
        .enumerate()
        .map(|(i, c)| if i < 6 || i >= limit { c } else { '*' })
        .collect()
}

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
