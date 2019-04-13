use std::borrow::Cow;
use std::path::PathBuf;
use crate::file::File;


#[derive(Debug)]
pub struct Target<'n> {
    pub name: &'n PathBuf,
    pub file: Option<File>,
}

impl<'n> Target<'n> {
    pub fn new(name: &'n PathBuf) -> Target<'n> {
        Target {
            name: name,
            file: File::new(name),
        }
    }

    pub fn long_name(&self) -> Cow<str> {
        if let Some(ref file) = self.file{
            file.abs_path.to_string_lossy()
        } else {
            self.short_name()
        }
    }

    pub fn short_name(&self) -> Cow<str> {
        self.name.to_string_lossy()
    }
}
