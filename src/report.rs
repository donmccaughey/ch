use crate::options::Options;
use std::borrow::Cow;
use std::env::args_os;
use std::path::Path;
use std::path::PathBuf;
use crate::file::File;
use std::error::Error;
use crate::changes::Changes;


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

    pub fn file_was_changed(&self, file: &'o File, changes: &'o Changes) {
        match self.options.verbose {
            0 => (),
            1 => println!("{}", self.file_name(file)),
            _ => println!("{}: {}", self.file_name(file), self.change_list(file, changes)),
        }
    }

    pub fn file_change_failed(&self, file: &'o File, error: &Box<Error>) {
        eprintln!("{}: {}", self.file_name(file), error);
    }

    pub fn target_is_missing(&self, name: &'o PathBuf) {
        println!("{}: {}: No such file or directory",
                 self.command, name.to_string_lossy());
    }

    fn change_list(&self, file: &'o File, changes: &'o Changes) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(new_owner) = changes.owner {
            let change = format!("owner {} -> {}",
                                 file.owner.name().to_string_lossy(),
                                 new_owner.name().to_string_lossy());
            parts.push(change);
        }
        if let Some(new_group) = changes.group {
            let change = format!("group {} -> {}",
                                 file.group.name().to_string_lossy(),
                                 new_group.name().to_string_lossy());
            parts.push(change);
        }
        if let Some(new_mode) = changes.mode {
            // TODO: mode 0100755 [-rwxr-xr-x ] -> 0100777 [-rwxrwxrwx ]
            let change = format!("mode {:07o} -> {:07o}", file.mode, new_mode);
            parts.push(change);
        }

        parts.join(", ")
    }

    fn file_name(&self, file: &'o File) -> Cow<str> {
        match self.options.verbose {
            0...2 => file.name.to_string_lossy(),
            _     => file.abs_path.to_string_lossy(),
        }
    }
}
