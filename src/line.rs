use std::io::Write;

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

    pub fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.as_str().as_bytes())
    }
}
