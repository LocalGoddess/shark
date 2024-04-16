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

    pub fn is_between_row(&self, start: &Self, end: &Self) -> bool {
        start.row <= self.row && end.row >= self.row
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

pub struct SharkSnippet<'a> {
    pub snippet: String,
    pub start_position: SourcePosition<'a>,
    pub end_position: SourcePosition<'a>,
}

impl<'a> SharkSnippet<'a> {
    pub fn new(
        source: &str,
        start_position: SourcePosition<'a>,
        end_position: SourcePosition<'a>,
    ) -> Self {
        let chars = source.chars();
        let mut snippet: String = String::new();

        let mut current_line = String::new();
        let mut current_position = SourcePosition::default();

        for current_char in chars {
            if current_position.is_between_row(&start_position, &end_position) {
                current_line.push(current_char);
            }

            current_position.next();
            if current_char == '\n' {
                Self::style_new_line(&mut snippet, &mut current_line, &mut current_position);
            }
        }

        Self {
            snippet,
            start_position,
            end_position,
        }
    }

    pub fn underline(
        &mut self,
        start_position: SourcePosition<'a>,
        underline_length: usize,
        message: Option<String>,
    ) {
        let raw_message = match message {
            Some(message) => message,
            None => String::new(),
        };
        let line_data = format!(
            "\n  | {}{} {}",
            " ".repeat(start_position.column - 1),
            "^".repeat(underline_length),
            raw_message
        );

        let mut new_snippet = String::new();
        let mut current_row = self.start_position.row;
        for char in self.snippet.chars() {
            if current_row == start_position.row && char == '\n' {
                new_snippet.push_str(line_data.as_str());
            }

            new_snippet.push(char);

            if char == '\n' {
                current_row += 1;
            }
        }
        self.snippet = new_snippet
    }

    fn style_new_line(
        snippet: &mut String,
        current_line: &mut String,
        current_position: &mut SourcePosition,
    ) {
        if !current_line.is_empty() {
            snippet.push_str(format!("{} | {}", current_position.row, current_line).as_str());
        }
        current_position.newline();
        current_line.clear();
    }
}
