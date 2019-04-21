use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use libc;
use users::get_group_by_gid;
use users::get_user_by_uid;
use users::Group;
use users::User;

use crate::changes::Change;
use crate::changes::ChMod;
use crate::changes::ChOwn;
use crate::options::Options;


#[derive(Debug)]
pub struct File<'o> {
    pub name: &'o PathBuf,
    pub abs_path: PathBuf,
    pub owner: User,
    pub group: Group,
    pub mode: libc::mode_t,
}

impl<'o> File<'o> {
    pub fn find(name: &'o PathBuf) -> Option<File> {
        let abs_path = match name.canonicalize() {
            Ok(abs_path) => abs_path,
            Err(_) => return None,
        };
        let metadata = match name.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return None,
        };
        let owner = match get_user_by_uid(metadata.uid()) {
            Some(owner) => owner,
            None => return None,
        };
        let group = match get_group_by_gid(metadata.gid()) {
            Some(group) => group,
            None => return None,
        };

        Some(File {
            name: name,
            abs_path: abs_path,
            owner: owner,
            group: group,
            mode: metadata.mode() as libc::mode_t,
        })
    }

    pub fn changes(&self, options: &'o Options) -> Vec<Change> {
        let mut changes: Vec<Change> = Vec::new();

        if let Some(chown) = ChOwn::new(self, options) {
            changes.push(Change::Owner(chown));
        }
        if let Some(chmod) = ChMod::new(self, options) {
            changes.push(Change::Mode(chmod));
        }

        changes
    }

    pub unsafe fn c_path(&self) -> *const libc::c_char {
        let bytes = self.abs_path.as_os_str().as_bytes();
        let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
        cstr.as_ptr()
    }
}
