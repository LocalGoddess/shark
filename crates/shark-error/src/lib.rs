use std::ops::Range;

use source::SourcePosition;

pub mod source;

pub struct SharkError<'a> {
    pub kind: SharkErrorKind,
    pub position: Range<SourcePosition<'a>>,

    pub message: &'a str,
    pub help: Option<&'a str>,
}

impl<'a> SharkError<'a> {
    pub fn new(
        kind: SharkErrorKind,
        position: Range<SourcePosition<'a>>,
        message: &'a str,
        help: Option<&'a str>,
    ) -> Self {
        Self {
            kind,
            position,
            message,
            help,
        }
    }

    pub fn get_header(&self) -> String {
        format!(
            "{} : \x1b[38;2;255;255;255m{}\x1b[0m\n found at: {}\n\n\n\n\n",
            self.kind.prefix(),
            self.message,
            self.position.start.stringify()
        )
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
