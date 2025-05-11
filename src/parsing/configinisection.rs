use crate::LineEnding;

use super::initoken::IniToken;

#[derive(Debug)]
pub struct ConfigIniSection<T: IniToken> {
    pub name: String,
    pub tokens: Vec<T>,
    pub line_ending: LineEnding,
    pub line_waste_prefix: String,
    pub line_waste_suffix: String,
}

impl<T: IniToken> ConfigIniSection<T> {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            tokens: Vec::new(),
            line_ending: LineEnding::Unknown,
            line_waste_prefix: String::new(),
            line_waste_suffix: String::new(),
        }
    }
    pub fn new_with_tokens(tokens: Vec<T>) -> Self {
        Self {
            name: String::new(),
            tokens,
            line_ending: LineEnding::Unknown,
            line_waste_prefix: String::new(),
            line_waste_suffix: String::new(),
        }
    }
    pub fn new_with_name_and_tokens(name: String, tokens: Vec<T>) -> Self {
        Self {
            name,
            tokens,
            line_ending: LineEnding::Unknown,
            line_waste_prefix: String::new(),
            line_waste_suffix: String::new(),
        }
    }

    pub fn clone(&mut self, template: ConfigIniSection<T>) -> ConfigIniSection<T> {
        let mut cloned = Self::new_with_name_and_tokens(template.name, Vec::new());
        cloned.line_ending = self.line_ending.clone();
        cloned.line_waste_prefix = self.line_waste_prefix.clone();
        cloned.line_waste_suffix = self.line_waste_suffix.clone();
        for
        cloned
    }
}
