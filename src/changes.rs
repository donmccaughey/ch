use std::error::Error;

use users::gid_t;
use users::Group;
use users::uid_t;
use users::User;

use crate::file::File;
use crate::libc_error::LibCError;
use crate::mode::ModeT;
use crate::options::Options;


pub struct AppliedChange<'o> {
    pub change: &'o Change<'o>,
    pub error: Option<Box<dyn Error>>,
}


pub enum Change<'o> {
    Mode(ChMod<'o>),
    Owner(ChOwn<'o>),
}

impl<'o> Change<'o> {
    pub fn apply(&self) -> AppliedChange {
        let result = match self {
            Change::Mode(ref chmod) => chmod.apply(),
            Change::Owner(ref chown) => chown.apply(),
        };
        AppliedChange {
            change: self,
            error: result.err(),
        }
    }
}


pub struct ChMod<'o> {
    pub file: &'o File<'o>,
    pub options: &'o Options,
    pub mode: ModeT,
}

impl<'o> ChMod<'o> {
    pub fn new(file: &'o File, options: &'o Options) -> Option<ChMod<'o>> {
        if let Some(ref mode_change) = options.mode_change {
            let new_mode = mode_change.apply(file.mode);
            if file.mode != new_mode {
                return Some(ChMod {
                    file: file,
                    options: options,
                    mode: new_mode,
                });
            }
        }
        None
    }

    pub fn apply(&self) -> Result<(), Box<dyn Error>> {
        if self.options.dry_run {
            return Ok(());
        } else {
            unsafe {
                match libc::chmod(self.file.c_path(), self.mode as u16) {
                    0 => Ok(()),
                    _ => return Err(Box::new(LibCError::from_errno())),
                }
            }
        }
    }
}


pub struct ChOwn<'o> {
    pub file: &'o File<'o>,
    pub options: &'o Options,
    pub owner: Option<&'o User>,
    pub uid: uid_t,
    pub group: Option<&'o Group>,
    pub gid: gid_t,
}

impl<'o> ChOwn<'o> {
    pub fn new(file: &'o File, options: &'o Options) -> Option<ChOwn<'o>> {
        let mut chown = ChOwn {
            file: file,
            options: options,
            owner: None,
            uid: file.owner.uid(),
            group: None,
            gid: file.group.gid(),
        };

        if let Some(ref new_owner) = options.owner {
            if chown.uid != new_owner.uid() {
                chown.owner = Some(new_owner);
                chown.uid = new_owner.uid();
            }
        }

        if let Some(ref new_group) = options.group {
            if chown.gid != new_group.gid() {
                chown.group = Some(new_group);
                chown.gid = new_group.gid();
            }
        }

        if chown.owner.is_some() || chown.group.is_some() {
            Some(chown)
        } else {
            None
        }
    }

    pub fn apply(&self) -> Result<(), Box<dyn Error>> {
        if self.options.dry_run {
            return Ok(());
        } else {
            unsafe {
                match libc::chown(self.file.c_path(), self.uid, self.gid) {
                    0 => Ok(()),
                    _ => return Err(Box::new(LibCError::from_errno())),
                }
            }
        }
    }
}
