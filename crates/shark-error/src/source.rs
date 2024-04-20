use std::{fmt::Display, path::Path};

/// Represents a position inside of a file. If file is of type `None`,
/// then this will just represent a line and column in any file.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SourcePosition<'a> {
    pub file: Option<&'a Path>,
    pub line: usize,
    pub column: usize,
}

impl<'a> SourcePosition<'a> {
    pub fn new(file: Option<&'a Path>, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }

    pub fn is_within_lines(&self, start: &Self, end: &Self) -> bool {
        if self.ensure_similar(start) && self.ensure_similar(end) {
            return self.line >= start.line && self.line <= end.line;
        }
        false
    }

    /// Ensures that these [`SourcePosition`]'s are of the same file
    fn ensure_similar(&self, other: &Self) -> bool {
        if self.file.is_some() && other.file.is_some() {
            return self.file == other.file;
        }
        true
    }
}

impl<'a> Display for SourcePosition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_str: &str = self.file.as_ref()
            .and_then(|x| x.to_str())
            .map_or_else(|| "unknown", |x| x);
        write!(f, "{}:{}:{}", path_str, self.line, self.column)
    }
}
