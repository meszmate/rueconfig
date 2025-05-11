use crate::LineEnding;

use super::textline::TextLine;

#[derive(Debug, Clone)]
pub struct MultiLineToken {
    pub lines: Vec<TextLine>,
}

impl MultiLineToken {
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
