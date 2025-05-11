use crate::LineEnding;

#[derive(Debug, Clone)]
pub struct LineToken {
    pub line_ending: LineEnding,
}

impl LineToken {
    pub fn new(line_ending: LineEnding) -> Self {
        Self { line_ending }
    }
}
