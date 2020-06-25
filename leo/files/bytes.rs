//! The program bytes file.

use crate::{directories::outputs::OUTPUTS_DIRECTORY_NAME, errors::BytesFileError};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static BYTES_FILE_EXTENSION: &str = ".bytes";

#[derive(Deserialize)]
pub struct BytesFile {
    pub package_name: String,
}

impl BytesFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the program bytes from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<Vec<u8>, BytesFileError> {
        let path = self.setup_file_path(path);

        Ok(fs::read(&path).map_err(|_| BytesFileError::FileReadError(path.clone()))?)
    }

    /// Writes the given program bytes to a file.
    pub fn write_to(&self, path: &PathBuf, bytes: Vec<u8>) -> Result<(), BytesFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(bytes.as_slice())?;

        log::info!("program bytes stored to {:?}", path);

        Ok(())
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, BYTES_FILE_EXTENSION)));
        }
        path
    }
}
