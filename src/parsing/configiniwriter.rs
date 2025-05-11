use crate::LineEnding;
use std::io;
use std::io::Write;

pub struct ConfigIniWriter<W: Write> {
    pub content_writer: W,
    pub line_ending: LineEnding,
}
impl<W: Write> ConfigIniWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            content_writer: writer,
            line_ending: LineEnding::Unknown,
        }
    }

    pub fn write(&mut self, content: String) -> io::Result<()> {
        self.content_writer.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn write_line_ending(&mut self, line_ending: LineEnding) -> std::io::Result<()> {
        line_ending.write_to(&mut self.content_writer)?;

        if self.line_ending == LineEnding::Unknown {
            self.line_ending = line_ending;
        }

        Ok(())
    }
}
