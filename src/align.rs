use std::fmt::Display;

/// Text console alignment
pub struct Alignment(char);

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_ref())
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment(' ')
    }
}

impl From<char> for Alignment {
    fn from(value: char) -> Self {
        Alignment(value)
    }
}

impl Alignment {
    pub fn fill<T: AsRef<str>>(&self, width: u32, text: T) -> String {
        // Center alignment
        let ltext = text.as_ref().len() as u32;
        let fill = (width / 2 - ltext / 2) as usize;

        // Filled
        let mut filled = self.to_string().repeat(fill);
        filled.push_str(text.as_ref());
        filled
    }
}
