use std::u32;


#[derive(Debug)]
pub struct Mode {
    pub additive_mask: u32,
    pub subtractive_mask: u32,
}

impl Mode {
    pub fn new(mode_str: &str) -> Option<Mode> {
        if let Ok(mode_bits) = u32::from_str_radix(mode_str, 8) {
            return Some(Mode {
                additive_mask: mode_bits,
                subtractive_mask: u32::MAX,
            })
        } else {
            None
        }
    }

    pub fn change(&self, mode_bits: u32) -> u32 {
        mode_bits & !self.subtractive_mask | self.additive_mask
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_octal() {
        let some_mode = Mode::new("0644");
        assert!(some_mode.is_some());
        let mode = some_mode.unwrap();
        assert_eq!(0o0644, mode.additive_mask);
        assert_eq!(0xffff_ffff, mode.subtractive_mask);
    }

    #[test]
    fn test_new_from_decimal() {
        let none_mode = Mode::new("0944");
        assert!(none_mode.is_none());
    }

    #[test]
    fn test_change() {
        let mode = Mode::new("0754").unwrap();
        assert_eq!(0o0754, mode.change(0o7777));
        assert_eq!(0o0754, mode.change(0o0777));
        assert_eq!(0o0754, mode.change(0o0755));
        assert_eq!(0o0754, mode.change(0o0644));
        assert_eq!(0o0754, mode.change(0o0000));
    }
}
