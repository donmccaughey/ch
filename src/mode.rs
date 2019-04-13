use std::u32;


pub type ModeT = u32;
pub type ModeMask = u32;


#[derive(Debug)]
pub struct Mode {
    pub additive_mask: ModeMask,
    pub subtractive_mask: ModeMask,
}

impl Mode {
    pub fn new(mode_str: &str) -> Option<Mode> {
        if let Ok(mode_bits) = ModeT::from_str_radix(mode_str, 8) {
            return Some(Mode {
                additive_mask: mode_bits,
                subtractive_mask: 0o0000_7777,
            })
        } else {
            None
        }
    }

    pub fn change(&self, mode_bits: ModeT) -> ModeT {
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
        assert_eq!(0o0000_0644, mode.additive_mask);
        assert_eq!(0o0000_7777, mode.subtractive_mask);
    }

    #[test]
    fn test_new_from_decimal() {
        let none_mode = Mode::new("0944");
        assert!(none_mode.is_none());
    }

    #[test]
    fn test_change() {
        let mode = Mode::new("0754").unwrap();
        assert_eq!(0o010_0754, mode.change(0o010_7777));
        assert_eq!(0o004_0754, mode.change(0o004_0777));
        assert_eq!(0o020_0754, mode.change(0o020_0755));
        assert_eq!(0o001_0754, mode.change(0o001_0644));
        assert_eq!(0o100_0754, mode.change(0o100_0000));
    }
}
