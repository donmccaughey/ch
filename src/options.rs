use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use structopt::StructOpt;
use users::get_group_by_name;
use users::get_user_by_name;
use users::Group;
use users::User;

use crate::mode::ModeChange;
use crate::target::Target;


fn get_group(group_name: &OsStr) -> Result<Group, OsString> {
    if let Some(group) = get_group_by_name(group_name) {
        Ok(group)
    } else {
        let mut message = group_name.to_os_string();
        message.push(": illegal group name");
        Err(message)
    }
}

fn get_mode_change(mode_name: &OsStr) -> Result<ModeChange, OsString> {
    if let Some(mode_str) = mode_name.to_str() {
        if let Some(mode) = ModeChange::new(mode_str) {
            return Ok(mode)
        }
    }
    let mut message = mode_name.to_os_string();
    message.push(": invalid mode");
    Err(message)
}

fn get_user(user_name: &OsStr) -> Result<User, OsString> {
    if let Some(user) = get_user_by_name(user_name) {
        Ok(user)
    } else {
        let mut message = user_name.to_os_string();
        message.push(": illegal user name");
        Err(message)
    }
}


#[derive(StructOpt, Debug)]
#[structopt()]
/// Change file properties
pub struct Options {
    #[structopt(name = "FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,

    #[structopt(short, long, parse(try_from_os_str = "get_group"))]
    /// Change the group of FILES to this group name or numeric ID
    pub group: Option<Group>,

    #[structopt(name = "mode", short, long, parse(try_from_os_str = "get_mode_change"))]
    /// Change the mode bits of FILES to this octal or symbolic mode
    pub mode_change: Option<ModeChange>,

    #[structopt(short, long, parse(try_from_os_str = "get_user"))]
    /// Change the owner of FILES to this user name or numeric ID
    pub owner: Option<User>,

    #[structopt(short, long, parse(from_occurrences))]
    /// Be verbose, show each modified file; specify twice to list old and
    /// new file properties, thrice to show absolute paths
    pub verbose: u8,
}

impl Options {
    pub fn new() -> Options {
        Options::from_args()
    }

    pub fn targets(&self) -> impl Iterator<Item=Target> {
        self.files.iter().map(Target::new)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_mode_change_ok() {
        let mode_str = OsStr::new("0754");
        let result = get_mode_change(mode_str);
        assert!(result.is_ok());
        let mode_change = result.unwrap();
        assert_eq!(0o0754, mode_change.apply(0o0000));
        assert_eq!(0o0754, mode_change.apply(0o7777));
    }

    #[test]
    fn test_get_mode_change_err() {
        let mode_str = OsStr::new("fubar");
        let result = get_mode_change(mode_str);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!("fubar: invalid mode", error);
    }
}
