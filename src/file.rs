use crate::options::Options;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use users::{User, Group, get_user_by_uid, get_group_by_gid};
use std::error::Error;


#[derive(Debug)]
pub struct File {
    pub abs_path: PathBuf,
    pub owner: User,
    pub group: Group,
    pub mode: u32,
}

impl File {
    pub fn new(name: &PathBuf) -> Option<File> {
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
            abs_path: abs_path,
            owner: owner,
            group: group,
            mode: metadata.mode(),
        })
    }

    pub fn change_properties(&self, options: &Options) -> Result<(), Box<Error>> {
        Ok(())
    }
}
