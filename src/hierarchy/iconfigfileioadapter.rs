use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub trait ConfigFileIOAdapter {
    fn get_directories(&self, pivot_path: &str) -> io::Result<Vec<String>>;

    fn open_read(&self, file_path: &str) -> io::Result<Box<dyn Read>>;

    fn open_write(&self, file_path: &str) -> io::Result<Box<dyn Write>>;
}