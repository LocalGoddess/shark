use std::{fmt::Display, ops::RangeInclusive, path::Path};

/// Represents a position within a source file. If the path is [Option::None], then this will just
/// represent a line and column in any source file
#[derive(Debug, Clone, Copy)]
pub struct SourcePosition<'position> {
    pub path: Option<&'position Path>,
    pub line: usize,
    pub column: usize,
}

impl<'position> SourcePosition<'position> {
    /// Creates a new [SourcePosition]
    /// Note! a line or column of zero can't exist in a [SourcePosition]. Both lines and columns
    /// start at one
    pub fn new(path: Option<&'position Path>, line: usize, column: usize) -> Self {
        if line == 0 || column == 0 {
            panic!("A SourcePosition can not have a line or column with a value of zero")
        }
        Self { path, line, column }
    }

    // This is done because [std::iter::Step] is currently in nightly. When that reaches full release
    // this will be removed
    pub fn into_iter(
        range: RangeInclusive<SourcePosition<'position>>,
    ) -> SourcePositionIterator<'position> {
        SourcePositionIterator {
            start: *range.start(),
            end: *range.end(),
            current: 0,
        }
    }
}

impl<'position> Display for SourcePosition<'position> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_str: &str = self
            .path
            .as_ref()
            .and_then(|x| x.to_str())
            .map_or_else(|| "unknown", |x| x);
        write!(formatter, "{}:{}:{}", path_str, self.line, self.column)
    }
}

impl<'position> PartialOrd for SourcePosition<'position> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // If both of these position are attached to files we make sure the files are the same
        // If one or both aren't tied to a file, we don't care
        if let (Some(self_path), Some(other_path)) = (&self.path, &other.path) {
            if self_path != other_path {
                return None;
            }
        }

        self.line.partial_cmp(&other.line)
    }
}

impl<'position> PartialEq for SourcePosition<'position> {
    fn eq(&self, other: &Self) -> bool {
        // Same as above, if both positions have a file attached to them, make sure they are the
        // smae file
        if let (Some(self_path), Some(other_path)) = (&self.path, &other.path) {
            if self_path != other_path {
                return false;
            }
        }
        self.line == other.line && self.column == other.column
    }
}

// This is done because [std::iter::Step] is currently in nightly. When that reaches full release
// this will be removed
pub struct SourcePositionIterator<'position> {
    start: SourcePosition<'position>,
    end: SourcePosition<'position>,
    current: usize,
}

impl<'position> Iterator for SourcePositionIterator<'position> {
    type Item = SourcePosition<'position>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current > self.end.line {
            return None;
        }
        Some(SourcePosition::new(
            self.start.path,
            self.start.line + self.current,
            1,
        ))
    }
}
