use std::io::{self, Write};

use super::configiniwriter::ConfigIniWriter;

pub trait IniToken<T: Write> {
    fn write(&self, writer: &mut ConfigIniWriter<T>) -> io::Result<()>;
}
