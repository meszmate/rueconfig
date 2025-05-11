use crate::LineEnding;

#[derive(Debug, Clone)]
pub struct TextLine {
    pub content: String,
    pub line_ending: LineEnding,
    pub is_null: bool,
}

impl TextLine {
    pub fn new(content: String, line_ending: LineEnding) -> Self {
        Self {
            content,
            line_ending,
            is_null: false,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}{}", self.content, self.line_ending.as_str())
    }
}
