use std::io::Write;

use crate::parsing::configiniwriter::ConfigIniWriter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Unknown,
    CR,
    LF,
    CRLF,
}

impl LineEnding {
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::CR => "\r",
            LineEnding::LF => "\n",
            LineEnding::CRLF => "\r\n",
            _ => "",
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut ConfigIniWriter<W>) -> std::io::Result<()> {
        writer.write(self.as_str().to_string())
    }
}
