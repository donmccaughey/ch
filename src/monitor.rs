use crate::options::Options;
use crate::target::Target;
use std::borrow::Cow;
use std::env::args_os;
use std::path::Path;


pub struct Monitor<'o> {
    command: String,
    options: &'o Options,
}

impl<'o> Monitor<'o> {
    pub fn new(options: &Options) -> Monitor {
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
        Monitor {
            command: command,
            options: options,
        }
    }

    pub fn changed_target(&self, target: &'o Target) {
        match self.options.verbose {
            0 => (),
            1 => println!("{}", self.target_name(target)),
            _ => println!("{}: {}", self.target_name(target), self.property_changes(target)),
        }
    }

    pub fn missing_target(&self, target: &'o Target) {
        println!("{}: {}: No such file or directory",
                 self.command, target.file.to_string_lossy());
    }

    fn property_changes(&self, target: &'o Target) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let (Some(ref from_owner), Some(ref to_owner)) = (&target.owner, &self.options.owner) {
            let change = format!("owner {} -> {}",
                                 from_owner.name().to_string_lossy(),
                                 to_owner.name().to_string_lossy());
            parts.push(change);
        }
        if let (Some(ref from_group), Some(ref to_group)) = (&target.group, &self.options.group) {
            let change = format!("group {} -> {}",
                                 from_group.name().to_string_lossy(),
                                 to_group.name().to_string_lossy());
            parts.push(change);
        }
        // TODO: mode 0100755 [-rwxr-xr-x ] -> 0100777 [-rwxrwxrwx ]
        parts.join(", ")
    }

    fn target_name(&self, target: &'o Target) -> Cow<str> {
        match self.options.verbose {
            0...2 => target.short_name(),
            _     => target.long_name(),
        }
    }
}
