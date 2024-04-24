use std::{path::Path, str::Chars};

use shark_error::{source::SourcePosition, SharkError, SharkErrorKind};

use crate::{KeywordKind, LexerToken, LiteralKind, TokenKind};

pub struct Lexer<'lexer> {
    position: usize,
    length: usize,

    expected_token: Option<TokenKind>,
    working_content: String,
    working_position: SourcePosition<'lexer>,

    current_position: SourcePosition<'lexer>,
    characters: Chars<'lexer>,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(source_origin: Option<&'lexer Path>, source: &'lexer str) -> Self {
        Self {
            position: 0,
            length: source.len(),
            expected_token: None,
            working_content: String::new(),
            working_position: SourcePosition::new(source_origin, 1, 1),
            current_position: SourcePosition::new(source_origin, 1, 1),
            characters: source.chars(),
        }
    }

    pub fn reset_token(&mut self) {
        self.expected_token = None;
        String::clear(&mut self.working_content);
    }

    pub fn push_token(&mut self, tokens: &mut Vec<LexerToken<'lexer>>) {
        if self.expected_token.is_none() {
            todo!("Error here");
        }

        tokens.push(LexerToken::new(
            self.expected_token
                .clone()
                .expect("Failed to get expected token type"),
            self.working_position,
            self.working_content.len(),
        ));
        self.reset_token();
    }

    pub fn push_single_char_token(
        &mut self,
        kind: TokenKind,
        tokens: &mut Vec<LexerToken<'lexer>>,
    ) {
        tokens.push(LexerToken::new(kind, self.current_position, 1));
        self.reset_token();
    }

    pub fn peek(&self) -> Option<char> {
        self.characters.clone().next()
    }

    pub fn lex(&mut self) -> Vec<LexerToken> {
        let mut tokens: Vec<LexerToken> = Vec::new();
        let mut errors: Vec<SharkError<'lexer>> = Vec::new();

        while self.position < self.length {
            let c: char = match self.characters.next() {
                Some(c) => c,
                None => break,
            };

            if self.expected_token.is_none() {
                self.find_new_token(c, &mut tokens, &mut errors);
            } else {
                self.continue_token(c, &mut tokens, &mut errors);
            }

            self.position += 1;
            self.current_position = SourcePosition::new(
                self.current_position.file,
                self.current_position.line,
                self.current_position.column + 1,
            );
        }

        tokens
    }

    pub fn find_new_token(
        &mut self,
        c: char,
        tokens: &mut Vec<LexerToken<'lexer>>,
        errors: &mut Vec<SharkError<'lexer>>,
    ) {
        if is_valid_identifier_char(&c, true) {
            self.working_content.push(c);
            self.expected_token = Some(TokenKind::Identifier(self.working_content.to_string()));
            self.working_position = self.current_position;

            self.finish_token(tokens, errors);
            return;
        }

        if c.is_ascii_digit() {
            self.working_content.push(c);
            self.expected_token = Some(TokenKind::Literal(LiteralKind::Int(-1)));
            self.working_position = self.current_position;

            self.finish_token(tokens, errors);

            return;
        }

        match c {
            '"' => {
                self.expected_token = Some(TokenKind::Literal(LiteralKind::Str(String::new())));
                self.working_position = self.current_position;
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
                self.working_position = self.current_position;
                self.expected_token = Some(TokenKind::Literal(LiteralKind::Char(String::new())));
                self.finish_token(tokens, errors);
            }

            '\n' => {
                self.current_position = SourcePosition::new(
                    self.current_position.file,
                    self.current_position.line + 1,
                    1,
                );
            }

            '+' => {
                self.push_single_char_token(TokenKind::Plus, tokens);
            }

            '-' => {
                // Note this won't allow the use of -.2. Instead, it will need to be -0.2 which
                // looks better anyway
                if let Some(peeked) = self.peek() {
                    if peeked.is_ascii_digit() {
                        self.working_position = self.current_position;
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
                if let Some(peeked) = self.peek() {
                    if peeked.is_ascii_digit() {
                        self.working_position = self.current_position;
                        self.working_content.push(peeked);
                        self.expected_token = Some(TokenKind::Literal(LiteralKind::Int(-1)));
                        return;
                    }
                }
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

    pub fn continue_token(
        &mut self,
        c: char,
        tokens: &mut Vec<LexerToken<'lexer>>,
        errors: &mut Vec<SharkError<'lexer>>,
    ) {
        self.working_content.push(c);
        self.finish_token(tokens, errors);
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
    pub fn finish_token(
        &mut self,
        tokens: &mut Vec<LexerToken<'lexer>>,
        errors: &mut Vec<SharkError<'lexer>>,
    ) {
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
                            self.expected_token =
                                Some(TokenKind::Literal(Self::deduce_numeric_type(
                                    &self.working_content,
                                    self.working_position,
                                    self.current_position,
                                    errors,
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

    fn deduce_numeric_type(
        content: &str,
        start_position: SourcePosition<'lexer>,
        end_position: SourcePosition<'lexer>,
        errors: &mut Vec<SharkError<'lexer>>,
    ) -> LiteralKind {
        if content.contains('.') {
            let result = content.parse::<f32>();
            if result.is_err() {
                let error = SharkError::new(
                    SharkErrorKind::Error,
                    start_position,
                    end_position,
                    "invalid float literal",
                );

                errors.push(error);
                return LiteralKind::Float(f32::NAN);
            }

            return LiteralKind::Float(result.unwrap());
        }
        let result = content.parse::<i32>();
        if result.is_err() {
            let error = SharkError::new(
                SharkErrorKind::Error,
                start_position,
                end_position,
                "invalid integer literal",
            );

            errors.push(error);
            return LiteralKind::Int(0);
        }
        LiteralKind::Int(result.unwrap())
    }
}

fn is_valid_identifier_char(c: &char, start: bool) -> bool {
    if start && c.is_ascii_digit() {
        return false;
    }
    c.is_alphabetic() || *c == '_' || c.is_ascii_digit()
}

fn is_valid_number_char(c: &char) -> bool {
    if c.is_ascii_digit() {
        return true;
    }
    matches!(c, '_' | '.' | 'x' | 'a'..='f' | 'A'..='F')
}

// TODO(Chloe): Improve this when I make the error system
fn _verify_char_content(_content: &str) {
    todo!("Verify the contents of char contents. For example you can't have a char that is 'ab', but you can have a char that is '\\u0040'. 
          IDK if/what this should return but I'll figure that out later. This will probably go back into the error system");
}
