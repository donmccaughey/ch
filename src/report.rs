use std::borrow::Cow;
use std::env::args_os;
use std::path::Path;
use std::path::PathBuf;

use crate::changes::AppliedChange;
use crate::changes::Change;
use crate::file::File;
use crate::options::Options;


pub struct Report<'o> {
    command: String,
    options: &'o Options,
}

impl<'o> Report<'o> {
    pub fn new(options: &Options) -> Report {
        let command = if let Some(arg) = args_os().nth(0) {
            let path = Path::new(&arg);
            if let Some(file_name) = path.file_name() {
                file_name.to_string_lossy().to_string()
            } else {
                path.to_string_lossy().to_string()
            }
        } else {
            "ch".to_string()
        };
        Report {
            command: command,
            options: options,
        }
    }

    pub fn changes_applied(&self, file: &'o File, applied_changes: Vec<AppliedChange>) {
        let successful_changes: Vec<_> = applied_changes.iter()
            .filter(|&applied_change| applied_change.error.is_none())
            .map(|successful_change| self.describe_change(successful_change))
            .collect();
        if !successful_changes.is_empty() {
            match self.options.verbose {
                0 => (),
                1 => println!("{}", self.file_name(file)),
                _ => println!("{}: {}", self.file_name(file), successful_changes.join(", ")),
            }
        }
        let failed_changes = applied_changes.iter()
            .filter(|&applied_change| applied_change.error.is_some());
        for applied_change in failed_changes {
            if let Some(ref error) = applied_change.error {
                eprintln!("{}: {}: ({}): {}", self.command, self.file_name(file), self.describe_change(&applied_change), error);
            }
        }
    }

    pub fn no_changes(&self, file: &'o File) {
        match self.options.verbose {
            0 => (),
            1 => println!("{}", self.file_name(file)),
            _ => println!("{}: no changes", self.file_name(file)),
        }
    }

    pub fn missing_target(&self, name: &'o PathBuf) {
        eprintln!("{}: {}: No such file or directory",
                  self.command, name.to_string_lossy());
    }

    fn describe_change(&self, applied_change: &AppliedChange) -> String {
        match applied_change.change {
            Change::Owner(ref chown) => {
                let mut parts: Vec<String> = Vec::new();
                if let Some(new_owner) = chown.owner {
                    let change = format!("owner {} -> {}",
                                         chown.file.owner.name().to_string_lossy(),
                                         new_owner.name().to_string_lossy());
                    parts.push(change);
                }
                if let Some(new_group) = chown.group {
                    let change = format!("group {} -> {}",
                                         chown.file.group.name().to_string_lossy(),
                                         new_group.name().to_string_lossy());
                    parts.push(change);
                }
                parts.join(", ")
            },
            Change::Mode(ref chmod) => {
                // TODO: mode 0100755 [-rwxr-xr-x ] -> 0100777 [-rwxrwxrwx ]
                format!("mode {:07o} -> {:07o}", chmod.file.mode, chmod.mode)
            },
        }
    }

    fn file_name(&self, file: &'o File) -> Cow<str> {
        match self.options.verbose {
            0...2 => file.name.to_string_lossy(),
            _     => file.abs_path.to_string_lossy(),
        }
    }
}
