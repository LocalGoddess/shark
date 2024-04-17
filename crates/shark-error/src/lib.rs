use source::SharkSnippet;

pub mod source;

pub struct SharkError<'a> {
    pub kind: SharkErrorKind,
    pub snippet: SharkSnippet<'a>,

    pub message: &'a str,
}

impl<'a> SharkError<'a> {
    pub fn new(kind: SharkErrorKind, snippet: SharkSnippet<'a>, message: &'a str) -> Self {
        Self {
            kind,
            snippet,
            message,
        }
    }

    pub fn get_header(&self) -> String {
        format!(
            "{} : \x1b[38;2;255;255;255m{}\n found at: {}\x1b[0m",
            self.kind.prefix(),
            self.message,
            self.snippet.start_position.stringify()
        )
    }

    pub fn get_error(&mut self) -> String {
        format!("{}\n{}", self.get_header(), self.snippet.snippet)
    }
}

pub enum SharkErrorKind {
    Error,
    Warn,
}

impl SharkErrorKind {
    pub fn color(&self) -> u32 {
        match self {
            Self::Error => 0xEB5E5E,
            Self::Warn => 0xEBC55E,
        }
    }

    pub fn prefix(&self) -> String {
        let color = self.color();

        let red = ((color >> 16) & 0xFF) as u8;
        let green = ((color >> 8) & 0xFF) as u8;
        let blue = (color & 0xFF) as u8;

        match self {
            Self::Error => format!("\x1b[1;38;2;{};{};{}merror!\x1b[0m", red, green, blue),
            Self::Warn => format!("\x1b[1;38;2;{};{};{}mwarning!\x1b[0m", red, green, blue),
        }
    }
}
