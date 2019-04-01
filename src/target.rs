use std::borrow::Cow;
use std::path::PathBuf;


#[derive(Debug)]
pub struct Target<'f> {
    pub file: &'f PathBuf,
    pub abs_path: Option<PathBuf>,
}

impl<'f> Target<'f> {
    pub fn new(file: &'f PathBuf) -> Target<'f> {
        let canonical_path = file.canonicalize();
        let abs_path = match canonical_path {
            Ok(abs_path) => Some(abs_path),
            Err(_)               => None,
        };
        Target {
            file: file,
            abs_path: abs_path,
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
