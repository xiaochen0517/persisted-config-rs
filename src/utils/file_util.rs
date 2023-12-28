use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub(crate) fn get_file_by_path(path: PathBuf, truncate: bool) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(truncate)
        .open(path)
}