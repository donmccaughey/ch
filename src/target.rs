use std::borrow::Cow;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use users::{User, Group, get_user_by_uid, get_group_by_gid};


#[derive(Debug)]
pub struct Target<'f> {
    pub file: &'f PathBuf,
    pub abs_path: Option<PathBuf>,
    pub owner: Option<User>,
    pub group: Option<Group>,
    pub mode: Option<u32>,
}

impl<'f> Target<'f> {
    pub fn new(file: &'f PathBuf) -> Target<'f> {
        let canonical_path = file.canonicalize();
        let abs_path = match canonical_path {
            Ok(abs_path) => Some(abs_path),
            Err(_)               => None,
        };

        let owner: Option<User>;
        let group: Option<Group>;
        let mode: Option<u32>;
        if let Ok(metadata) = file.metadata() {
            owner = get_user_by_uid(metadata.uid());
            group = get_group_by_gid(metadata.gid());
            mode = Some(metadata.mode());
        } else {
            owner = None;
            group = None;
            mode = None;
        }

        Target {
            file: file,
            abs_path: abs_path,
            owner: owner,
            group: group,
            mode: mode,
        }
    }

    pub fn change_properties(&self) {

    }

    pub fn exists(&self) -> bool {
        match self.abs_path {
            Some(_) => true,
            None => false,
        }
    }

    pub fn long_name(&self) -> Cow<str> {
        if let Some(ref abs_path) = self.abs_path {
            abs_path.to_string_lossy()
        } else {
            self.short_name()
        }
    }

    pub fn short_name(&self) -> Cow<str> {
        self.file.to_string_lossy()
    }
}
