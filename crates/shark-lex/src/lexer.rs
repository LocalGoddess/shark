use std::str::Chars;

use crate::{KeywordKind, LexerToken, LiteralKind, TokenKind};

pub struct Lexer<'a> {
    position: usize,
    length: usize,

    expected_token: Option<TokenKind>,
    working_row_col: (usize, usize),
    working_content: String,

    characters: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            position: 0,
            length: source.len(),
            expected_token: None,
            working_row_col: (0, 0),
            working_content: String::new(),
            characters: source.chars(),
        }
    }

    pub fn reset_token(&mut self) {
        self.expected_token = None;
        String::clear(&mut self.working_content);
    }

    pub fn push_token(&self, tokens: &mut Vec<LexerToken>) {
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
                self.continue_token(c);
            }

            self.position += 1;
        }

        tokens
    }

    pub fn find_new_token(&mut self, c: char, tokens: &mut Vec<LexerToken>) {
        if is_valid_identifier_char(&c, true) {
            self.working_content.push(c);
            self.expected_token = Some(TokenKind::Identifier(self.working_content.to_string()));

            self.finish_token(tokens);
        }

        if c.is_numeric() {
            self.working_content.push(c);
            self.expected_token = Some(TokenKind::Literal(LiteralKind::Int(-1)));

            self.finish_token(tokens);
        }
    }

    pub fn continue_token(&mut self, _c: char) {}

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
                        return true;
                    }
                    false
                }

                LiteralKind::Char(_) => {
                    if peeked == '\'' {
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

        if let Some(token) = self.expected_token {
            match token {
                TokenKind::Identifier(_) => {
                    if let Some(kw) = KeywordKind::is_keyword(&self.working_content) {
                        self.expected_token = Some(TokenKind::Keyword(kw));
                    }

                    if let Some(possible_boolean) =
                        LiteralKind::bool_from_string(&self.working_content)
                    {
                        self.expected_token = Some(TokenKind::Literal(possible_boolean))
                    }

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
                            self.expected_token =
                                Some(TokenKind::Literal(LiteralKind::Str(&self.working_content)));
                            self.push_token(tokens);
                        }

                        LiteralKind::Char(_) => {
                            self.expected_token =
                                Some(TokenKind::Literal(LiteralKind::Char(&self.working_content)));
                            self.push_token(tokens);
                        }

                        _ => {
                            // Unreachable
                            return;
                        }
                    }
                }

                _ => {
                    // Unreachable
                    return;
                }
            }
        }
    }
}

pub fn is_valid_identifier_char(_c: &char, _start: bool) -> bool {
    todo!("Finish this");
}

pub fn is_valid_number_char(_c: &char) -> bool {
    todo!("Finish this")
}

// TODO(Chloe): Improve this when I make the error system
pub fn deduce_numeric_type(content: &str) -> LiteralKind {
    if content.contains(".") {
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
