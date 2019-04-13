use std::error::Error;
use std::fs::Permissions;
use std::fs::set_permissions;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use users::get_group_by_gid;
use users::get_user_by_uid;
use users::Group;
use users::User;

use crate::changes::Changes;
use crate::mode::ModeT;
use crate::options::Options;


#[derive(Debug)]
pub struct File<'o> {
    pub name: &'o PathBuf,
    pub abs_path: PathBuf,
    pub owner: User,
    pub group: Group,
    pub mode: ModeT,
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
            mode: metadata.mode(),
        })
    }

    pub fn change_properties(&self, options: &'o Options) -> Result<Changes<'o>, Box<Error>> {
        let mut changes = Changes {
            owner: None,
            group: None,
            mode: None,
        };

        if let Some(ref new_owner) = options.owner {
            if self.owner.uid() != new_owner.uid() {
                if !options.dry_run {
                    // TODO: change owner
                }
                changes.owner = Some(new_owner);
            }
        }

        if let Some(ref new_group) = options.group {
            if self.group.gid() != new_group.gid() {
                if !options.dry_run {
                    // TODO: change group
                }
                changes.group = Some(new_group);
            }
        }

        if let Some(ref mode_change) = options.mode_change {
            let new_mode = mode_change.apply(self.mode);
            if self.mode != new_mode {
                if !options.dry_run {
                    match set_permissions(&self.abs_path, Permissions::from_mode(new_mode)) {
                        Ok(_) => (),
                        Err(error) => return Err(Box::new(error)),
                    }
                }
                changes.mode = Some(new_mode);
            }
        }

        Ok(changes)
    }
}
