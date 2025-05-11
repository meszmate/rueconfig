use std::io;
use std::io::Write;

use super::{configiniwriter::ConfigIniWriter, initoken::IniToken, linetoken::LineToken};

#[derive(Debug, Clone)]
pub struct TextToken {
    line_token: LineToken,
    text: String,
}

impl<T: Write> IniToken<T> for TextToken {
    fn write(&self, writer: &mut ConfigIniWriter<T>) -> io::Result<()> {
        writer.write(self.text.clone())
    }
}
