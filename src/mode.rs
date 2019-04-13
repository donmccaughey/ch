use std::u32;


pub type ModeT = u32;
pub type ModeMask = u32;


#[derive(Debug)]
pub struct ModeChange {
    additive: ModeMask,
    subtractive: ModeMask,
}

impl ModeChange {
    pub fn new(mode_str: &str) -> Option<ModeChange> {
        if let Ok(mode_bits) = ModeT::from_str_radix(mode_str, 8) {
            return Some(ModeChange {
                additive: mode_bits,
                subtractive: 0o7777,
            })
        } else {
            None
        }
    }

    pub fn apply(&self, mode: ModeT) -> ModeT {
        mode & !self.subtractive | self.additive
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_octal() {
        let option = ModeChange::new("0644");
        assert!(option.is_some());
        let mode_change = option.unwrap();
        assert_eq!(0o0644, mode_change.additive);
        assert_eq!(0o7777, mode_change.subtractive);
    }

    #[test]
    fn test_new_from_decimal() {
        let option = ModeChange::new("944");
        assert!(option.is_none());
    }

    #[test]
    fn test_apply() {
        let mode_change = ModeChange::new("0754").unwrap();
        assert_eq!(0o010_0754, mode_change.apply(0o010_7777));
        assert_eq!(0o004_0754, mode_change.apply(0o004_0777));
        assert_eq!(0o020_0754, mode_change.apply(0o020_0755));
        assert_eq!(0o001_0754, mode_change.apply(0o001_0644));
        assert_eq!(0o100_0754, mode_change.apply(0o100_0000));
    }
}
