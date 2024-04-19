use std::{
    io::{self, Write},
    ops::Range,
};

use source::SourcePosition;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub mod source;

pub struct SharkError<'a> {
    pub kind: SharkErrorKind,
    pub position: Range<SourcePosition<'a>>,
    pub message: &'a str,
    pub help_message: Option<&'a str>,
}

impl<'a> SharkError<'a> {
    pub fn new(
        kind: SharkErrorKind,
        position: Range<SourcePosition<'a>>,
        message: &'a str,
    ) -> Self {
        Self {
            kind,
            position,
            message,
            help_message: None,
        }
    }

    pub fn supply_help(&mut self, help_message: &'a str) {
        self.help_message = Some(help_message);
    }

    pub fn print_error(&self) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(
            ColorSpec::new()
                .set_fg(Some(self.kind.highlight_color()))
                .set_bold(true),
        )?;
        write!(&mut stdout, "{}! ", self.kind.prefix())?;

        stdout.reset()?;
        writeln!(
            &mut stdout,
            "{}\n  at {}",
            self.message, self.position.start
        )?;

        self.print_snippet(&mut stdout)?;

        Ok(())
    }

    fn print_snippet(&self, stream: &mut StandardStream) -> io::Result<()> {
        let file = match self.position.start.file {
            Some(path) => path,
            None => match self.position.end.file {
                Some(path) => path,
                None => panic!("Couldn't find file to get source snippet from"),
            },
        };
        let content = match std::fs::read_to_string(file) {
            Ok(content) => String::from(content),
            Err(_) => String::new(),
        };

        let mut current_position = SourcePosition::new(None, 1, 1);
        let mut line_data: String = String::new();

        for char in content.chars() {
            if current_position.is_within_lines(&self.position.start, &self.position.end) {
                line_data.push(char);
                if char == '\n' {
                    writeln!(stream, "{} |  {}", current_position.line, line_data)?;
                    line_data.clear();
                }
            }
            current_position =
                SourcePosition::new(None, current_position.line, current_position.column + 1);
        }

        let help = match self.help_message {
            Some(message) => format!("help: {}", message),
            None => String::new(),
        };
        write!(stream, "  |  ")?;

        stream.set_color(ColorSpec::new().set_fg(Some(self.kind.highlight_color())))?;
        write!(
            stream,
            "{}{} {}\n",
            " ".repeat(self.position.start.column - 1),
            "^".repeat(self.position.end.column - self.position.start.column),
            help
        )?;

        stream.reset()?;
        Ok(())
    }
}

pub enum SharkErrorKind {
    Error,
    Warn,
}

impl SharkErrorKind {
    pub fn highlight_color(&self) -> Color {
        match self {
            Self::Error => Color::Rgb(235, 94, 94),
            Self::Warn => Color::Rgb(235, 197, 94),
        }
    }

    pub fn prefix(&self) -> String {
        match self {
            Self::Error => String::from("error"),
            Self::Warn => String::from("warn"),
        }
    }
}
