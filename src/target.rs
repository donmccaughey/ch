use std::path::PathBuf;
use crate::file::File;


#[derive(Debug)]
pub enum Target<'o> {
    Found(File<'o>),
    Missing(&'o PathBuf),
}

impl<'o> Target<'o> {
    pub fn new(name: &'o PathBuf) -> Target<'o> {
        match File::find(name) {
            Some(file) => Target::Found(file),
            None            => Target::Missing(name),
        }
    }
}
