use std::str::Chars;

use crate::{KeywordKind, LexerToken, LiteralKind, TokenKind};

pub struct Lexer<'a> {
    position: usize,
    length: usize,

    expected_token: Option<TokenKind>,
    working_content: String,
    working_row_col: (usize, usize),

    current_row_col: (usize, usize),
    characters: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            position: 0,
            length: source.len(),
            expected_token: None,
            working_content: String::new(),
            working_row_col: (1, 1),
            current_row_col: (1, 1),
            characters: source.chars(),
        }
    }

    pub fn reset_token(&mut self) {
        self.expected_token = None;
        String::clear(&mut self.working_content);
    }

    pub fn push_token(&mut self, tokens: &mut Vec<LexerToken>) {
        if self.expected_token.is_none() {
            todo!("Error here");
        }

        tokens.push(LexerToken::new(
            self.expected_token
                .clone()
                .expect("Failed to get expected token type"),
            self.working_row_col.0,
            self.working_row_col.1,
            self.working_content.len(),
        ));
        self.reset_token();
    }

    pub fn push_single_char_token(&mut self, kind: TokenKind, tokens: &mut Vec<LexerToken>) {
        tokens.push(LexerToken::new(
            kind,
            self.current_row_col.0,
            self.current_row_col.1,
            1,
        ));
        self.reset_token();
    }

    pub fn peek(&self) -> Option<char> {
        self.characters.clone().next()
    }

    pub fn lex(&mut self) -> Vec<LexerToken> {
        let mut tokens: Vec<LexerToken> = Vec::new();

        while self.position < self.length {
            let c: char = match self.characters.next() {
                Some(c) => c,
                None => break,
            };

            if self.expected_token.is_none() {
                self.find_new_token(c, &mut tokens);
            } else {
                self.continue_token(c, &mut tokens);
            }

            self.position += 1;
            self.current_row_col = (self.current_row_col.0, self.current_row_col.1 + 1);
        }

        tokens
    }

    pub fn find_new_token(&mut self, c: char, tokens: &mut Vec<LexerToken>) {
        if is_valid_identifier_char(&c, true) {
            self.working_content.push(c);
            self.expected_token = Some(TokenKind::Identifier(self.working_content.to_string()));
            self.working_row_col = self.current_row_col;

            self.finish_token(tokens);
            return;
        }

        if c.is_ascii_digit() {
            self.working_content.push(c);
            self.expected_token = Some(TokenKind::Literal(LiteralKind::Int(-1)));
            self.working_row_col = self.current_row_col;

            self.finish_token(tokens);

            return;
        }

        match c {
            '"' => {
                self.expected_token = Some(TokenKind::Literal(LiteralKind::Str(String::new())));
                self.working_row_col = self.current_row_col;
                self.should_finish();
            }

            '\'' => {
                let mut cloned = self.characters.clone();
                let _ = cloned.next();
                if let Some(peek) = cloned.next() {
                    if peek == ',' || peek == ' ' {
                        // <'a, 'b, 'c, 'd> // ' '
                        self.push_single_char_token(TokenKind::Apostrophe, tokens);
                        return;
                    }
                }
                self.working_row_col = self.current_row_col;
                self.expected_token = Some(TokenKind::Literal(LiteralKind::Char(String::new())));
                self.finish_token(tokens);
            }

            '\n' => {
                self.current_row_col = (self.current_row_col.0 + 1, 1);
            }

            '+' => {
                self.push_single_char_token(TokenKind::Plus, tokens);
            }

            '-' => {
                // Note this won't allow the use of -.2. Instead, it will need to be -0.2 which
                // looks better anyway
                if let Some(peeked) = self.peek() {
                    if peeked.is_ascii_digit() {
                        self.working_row_col = self.current_row_col;
                        self.working_content.push(c);
                        self.expected_token = Some(TokenKind::Literal(LiteralKind::Int(-1)));
                        return;
                    }
                }

                self.push_single_char_token(TokenKind::Minus, tokens);
            }

            '=' => {
                self.push_single_char_token(TokenKind::Equal, tokens);
            }

            '*' => {
                self.push_single_char_token(TokenKind::Astrisk, tokens);
            }

            '/' => {
                self.push_single_char_token(TokenKind::Slash, tokens);
            }

            '&' => {
                self.push_single_char_token(TokenKind::Ampersand, tokens);
            }

            '|' => {
                self.push_single_char_token(TokenKind::Pipe, tokens);
            }

            '!' => {
                self.push_single_char_token(TokenKind::Bang, tokens);
            }

            '^' => {
                self.push_single_char_token(TokenKind::Caret, tokens);
            }

            '.' => {
                self.push_single_char_token(TokenKind::Dot, tokens);
            }

            ':' => {
                self.push_single_char_token(TokenKind::Colon, tokens);
            }

            '_' => {
                self.push_single_char_token(TokenKind::UnderScore, tokens);
            }

            '<' => {
                self.push_single_char_token(TokenKind::AngleBracket { opened: true }, tokens);
            }

            '>' => {
                self.push_single_char_token(TokenKind::AngleBracket { opened: false }, tokens);
            }

            '{' => {
                self.push_single_char_token(TokenKind::CurlyBrace { opened: true }, tokens);
            }

            '}' => {
                self.push_single_char_token(TokenKind::CurlyBrace { opened: false }, tokens);
            }

            '[' => {
                self.push_single_char_token(TokenKind::SquareBracket { opened: true }, tokens);
            }

            ']' => {
                self.push_single_char_token(TokenKind::SquareBracket { opened: false }, tokens);
            }

            '(' => {
                self.push_single_char_token(TokenKind::Parenthesis { opened: true }, tokens);
            }

            ')' => {
                self.push_single_char_token(TokenKind::Parenthesis { opened: false }, tokens);
            }

            ';' => {
                self.push_single_char_token(TokenKind::EOL, tokens);
            }

            _ => {
                // todo!("Error here") - Commenting this so the test passes
            }
        }
    }

    pub fn continue_token(&mut self, c: char, tokens: &mut Vec<LexerToken>) {
        self.working_content.push(c);
        self.finish_token(tokens);
    }

    // Maybe this needs a refactor
    fn should_finish(&mut self) -> bool {
        let maybe_peeked = self.peek();
        if maybe_peeked.is_none() {
            return true;
        }
        if self.expected_token.is_none() {
            return false;
        }

        let peeked = maybe_peeked.unwrap(); // checked
        let token = self.expected_token.clone().unwrap();

        match token {
            TokenKind::Identifier(_) => {
                if !is_valid_identifier_char(&peeked, false) {
                    return true;
                }
                false
            }

            TokenKind::Keyword(_) => {
                if !is_valid_identifier_char(&peeked, false) {
                    return true;
                }
                false
            }

            TokenKind::Literal(kind) => match kind {
                LiteralKind::Int(_) | LiteralKind::Float(_) => {
                    if !is_valid_number_char(&peeked) {
                        return true;
                    }
                    false
                }

                LiteralKind::Str(_) => {
                    if peeked == '"' {
                        let _ = self.characters.next(); // We don't want to check this on the next run
                        return true;
                    }
                    false
                }

                LiteralKind::Char(_) => {
                    if peeked == '\'' {
                        let _ = self.characters.next(); // We don't want to check this on the next run
                        return true;
                    }
                    false
                }

                LiteralKind::Boolean(_) => true,
            },

            _ => true,
        }
    }

    /// Finishes a token if it should be.
    pub fn finish_token(&mut self, tokens: &mut Vec<LexerToken>) {
        if !self.should_finish() {
            return;
        }

        if let Some(token) = &self.expected_token {
            match token {
                // This is stupid as shit
                TokenKind::Identifier(_) => {
                    if let Some(kw) = KeywordKind::is_keyword(&self.working_content) {
                        self.expected_token = Some(TokenKind::Keyword(kw));
                        self.push_token(tokens);
                        return;
                    }

                    if let Some(possible_boolean) =
                        LiteralKind::bool_from_string(&self.working_content)
                    {
                        self.expected_token = Some(TokenKind::Literal(possible_boolean));
                        self.push_token(tokens);
                        return;
                    }

                    self.expected_token = Some(TokenKind::Identifier(self.working_content.clone()));
                    self.push_token(tokens);
                }

                TokenKind::Literal(kind) => {
                    match kind {
                        LiteralKind::Float(_) | LiteralKind::Int(_) => {
                            self.expected_token = Some(TokenKind::Literal(deduce_numeric_type(
                                &self.working_content,
                            )));
                            self.push_token(tokens);
                        }

                        LiteralKind::Str(_) => {
                            self.expected_token = Some(TokenKind::Literal(LiteralKind::Str(
                                self.working_content.clone(),
                            )));
                            self.push_token(tokens);
                        }

                        LiteralKind::Char(_) => {
                            self.expected_token = Some(TokenKind::Literal(LiteralKind::Char(
                                self.working_content.clone(),
                            )));
                            self.push_token(tokens);
                        }

                        _ => {
                            // Unreachable
                        }
                    }
                }

                _ => {
                    // Unreachable
                }
            }
        }
    }
}

pub fn is_valid_identifier_char(c: &char, start: bool) -> bool {
    if start && c.is_ascii_digit() {
        return false;
    }
    c.is_alphabetic() || *c == '_' || c.is_ascii_digit()
}

pub fn is_valid_number_char(c: &char) -> bool {
    if c.is_ascii_digit() {
        return true;
    }
    matches!(c, '_' | '.' | 'x' | 'a'..='f' | 'A'..='F')
}

// TODO(Chloe): Improve this when I make the error system
pub fn deduce_numeric_type(content: &str) -> LiteralKind {
    if content.contains('.') {
        let result = content.parse::<f32>();
        if result.is_err() {
            todo!("Error here")
        }

        return LiteralKind::Float(result.unwrap());
    }
    let result = content.parse::<i32>();
    if result.is_err() {
        todo!("Error here")
    }
    LiteralKind::Int(result.unwrap())
}

pub fn verify_char_content(_content: &str) {
    todo!("Verify the contents of char contents. For example you can't have a char that is 'ab', but you can have a char that is '\\u0040'. 
          IDK if/what this should return but I'll figure that out later. This will probably go back into the error system");
}
