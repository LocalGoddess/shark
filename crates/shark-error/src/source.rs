use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SourcePosition<'a> {
    pub origin: Option<&'a Path>,
    pub row: usize,
    pub column: usize,
}

impl<'a> SourcePosition<'a> {
    pub fn new(origin_file: Option<&'a Path>, row: usize, column: usize) -> Self {
        Self {
            origin: origin_file,
            row,
            column,
        }
    }

    /// Advances forward by one column. Does not check for newlines
    pub fn next(&mut self) {
        self.column += 1;
    }

    /// Advances forward by one row
    pub fn newline(&mut self) {
        self.row += 1;
        self.column = 1;
    }

    pub fn stringify(&self) -> String {
        if self.origin.is_none() {
            return format!("unknown:{}:{}", self.row, self.column);
        }
        format!(
            "{}:{}:{}",
            self.origin.unwrap().display(),
            self.row,
            self.column
        )
    }
}

impl<'a> Default for SourcePosition<'a> {
    fn default() -> Self {
        Self::new(None, 1, 1)
    }
}

impl<'a> PartialOrd for SourcePosition<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.row == other.row {
            if self.column == other.column {
                return Some(std::cmp::Ordering::Equal);
            }

            if self.column > other.column {
                return Some(std::cmp::Ordering::Greater);
            }
        }

        if self.row > other.row {
            return Some(std::cmp::Ordering::Greater);
        }

        Some(std::cmp::Ordering::Less)
    }
}
