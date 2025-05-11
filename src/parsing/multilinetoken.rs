use super::configiniwriter::ConfigIniWriter;
use crate::LineEnding;
use std::io;
use std::io::Write;

use super::{initoken::IniToken, textline::TextLine};

#[derive(Debug, Clone)]
pub struct MultiLineToken {
    pub lines: Vec<TextLine>,
}

impl MultiLineToken {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn new_with_data(lines: Vec<String>, line_ending: LineEnding) -> Self {
        let mut new = Self::new();
        new.add_lines(lines, line_ending);
        new
    }

    pub fn add_line(&mut self, content: String, line_ending: LineEnding) {
        self.lines.push(TextLine::new(content, line_ending));
    }
    pub fn add_lines(&mut self, contents: Vec<String>, line_ending: LineEnding) {
        for c in contents {
            self.lines.push(TextLine::new(c, line_ending));
        }
    }
    pub fn get_string_lines(&self) -> Vec<String> {
        let mut strings: Vec<String> = Vec::new();
        for l in &self.lines {
            strings.push(l.to_string());
        }
        strings
    }
}

impl<T: Write> IniToken<T> for MultiLineToken {
    fn write(&self, writer: &mut ConfigIniWriter<T>) -> io::Result<()> {
        for l in &self.lines {
            if !l.is_null {
                writer.write(l.content.clone());
                l.line_ending.write_to(writer);
            }
        }
        Ok(())
    }
}
