#[macro_use]
pub mod macros;

use std::{path::Path, str::Chars};

use shark_core::source::SourcePosition;
use token::{CommentKind, KeywordKind, LexerToken, LiteralKind, TokenKind};

pub mod error;
pub mod token;

#[cfg(test)]
pub mod test;

/// Step one of compilation. Turns the written code into [LexerToken]s to be used by the parser
#[derive(Debug)]
pub struct Lexer<'lexer> {
    /// Basic Lexer State
    pub source: Chars<'lexer>,
    pub source_length: usize,
    pub current_position: SourcePosition<'lexer>,
    pub completed_tokens: Vec<LexerToken<'lexer>>,

    /// Current Token State
    pub token_start_position: Option<SourcePosition<'lexer>>,
    pub token_content: String,
    pub token_inferred_kind: Option<TokenKind>,

    pub in_comment: Option<CommentKind>,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(path: Option<&'lexer Path>, src: &'lexer str) -> Self {
        Self {
            source: src.chars(),
            source_length: src.len(),
            current_position: SourcePosition::new(path, 1, 1),
            completed_tokens: Vec::new(),

            token_start_position: None,
            token_content: String::new(),
            token_inferred_kind: None,

            in_comment: None,
        }
    }

    /// Creates the initial state when finding a [LexerToken]
    fn start_token(&mut self, kind: TokenKind, initial_data: Option<char>) {
        self.token_inferred_kind = Some(kind);
        self.token_start_position = Some(self.current_position);
        if let Some(initial_data) = initial_data {
            self.token_content.push(initial_data);
        }
    }

    /// Finish the active [LexerToken] and adds it to the [Vec] of completed tokens. Then it resets
    /// the state of the [Lexer]
    fn push_token(&mut self) {
        if self.token_inferred_kind.is_none() {
            panic!("invalid lexer state to push a token"); // See above comment
        }

        self.completed_tokens.push(LexerToken {
            kind: self
                .token_inferred_kind
                .clone()
                .expect("expected a [TokenKind] found [None] while creating a [LexerToken]"),
            position: self
                .token_start_position
                .expect("expected a [SourcePosition] found [None] while creating a [LexerToken]")
                ..=self.current_position,
            length: self.token_content.len(),
        });
        self.reset_token_state();
    }

    /// Creates then pushes a "small token", that is any token which is 1-2 characters in length.
    /// This will also reset the state of the [Lexer]
    fn push_small_token(&mut self, current_character: char, peek: Option<char>) {
        if let Some(kind) = TokenKind::create_grammar_token(&current_character, peek.as_ref()) {
            self.start_token(kind.clone(), Some(current_character));

            if TokenKind::get_grammar_token_length(&kind) > 1 {
                self.token_content.push(peek.unwrap());
                self.source.next(); // consume
            }
            self.push_token();
            return;
        }
        todo!("error here");
    }

    fn reset_token_state(&mut self) {
        self.token_start_position = None;
        self.token_inferred_kind = None;
        self.token_content = String::new();
    }

    fn peek(&mut self) -> Option<char> {
        self.source.clone().next()
    }

    pub fn lex(&mut self) {
        while let Some(current_character) = self.source.next() {
            if self.in_comment.is_some() {
                self.handle_comment(current_character);
                continue; // If we are in a comment we want to avoid starting new tokens
            }

            if self.token_inferred_kind.is_none() {
                // Find a new token to infer
                self.infer_token(current_character)
            }
            // The check to see if we have an inferenced token is going to be here because it would
            // stop duplicate code checking if a token needs to end :)
            self.continue_token(&current_character);
            if current_character == '\n' {
                self.current_position.next_line();
            } else {
                self.current_position.next_column();
            }
        }
    }

    /// Logic for checking when we need to exit a comment
    fn handle_comment(&mut self, current_character: char) {
        match self.in_comment {
            Some(CommentKind::SingleLine) => {
                if current_character == '\n' {
                    self.in_comment = None;
                }
            }
            Some(CommentKind::MultiLine) => {
                if let Some(peek) = self.peek() {
                    if current_character == '*' && peek == '/' {
                        self.in_comment = None;
                        self.source.next(); // conusme the slash
                    }
                }
            }
            None => {}
        }
    }

    fn infer_token(&mut self, current_character: char) {
        match current_character {
            '"' => {
                self.start_token(TokenKind::Literal(LiteralKind::Str(String::new())), None);
            }
            '\'' => {
                self.start_token(TokenKind::Literal(LiteralKind::Char('\0')), None);
            }
            '-' => {
                let peek = self.peek();
                if peek.is_some_and(|x| x.is_ascii_digit()) {
                    self.start_token(TokenKind::Literal(LiteralKind::Int8(0)), None);
                    return;
                }
                self.push_small_token(current_character, peek);
            }
            '.' => {
                let peek = self.peek();
                if peek.is_some_and(|x| x.is_ascii_digit()) {
                    self.start_token(TokenKind::Literal(LiteralKind::Float32(0.0)), None);
                    return;
                }
                self.push_small_token(current_character, peek);
            }
            '/' => {
                if let Some(peek) = self.peek() {
                    match peek {
                        '/' => {
                            self.in_comment = Some(CommentKind::SingleLine);
                            self.source.next();
                            return;
                        }
                        '*' => {
                            self.in_comment = Some(CommentKind::MultiLine);
                            self.source.next();
                            return;
                        }
                        _ => {}
                    }
                }

                let peek = self.peek();
                if let Some(grammar_token) =
                    TokenKind::create_grammar_token(&current_character, peek.as_ref())
                {
                    self.start_token(grammar_token.clone(), Some(current_character));
                    if grammar_token.get_grammar_token_length() > 1 {
                        self.token_content.push(peek.unwrap());
                        self.source.next();
                    }
                    self.push_token();
                }
            }

            ' ' => {}
            '\n' => {}
            _ => {
                let peek = self.peek();
                if TokenKind::is_valid_identifier_character(true, &current_character) {
                    self.start_token(TokenKind::Identifier(String::new()), None);
                    return;
                }

                if current_character.is_ascii_digit() {
                    self.start_token(TokenKind::Literal(LiteralKind::Int8(0)), None);
                    return;
                }

                if let Some(grammar_token) =
                    TokenKind::create_grammar_token(&current_character, peek.as_ref())
                {
                    self.start_token(grammar_token.clone(), Some(current_character));
                    if grammar_token.get_grammar_token_length() > 1 {
                        self.token_content.push(peek.unwrap());
                        self.source.next();
                    }
                    self.push_token();
                    return;
                }
                todo!("error: disallowed character")
            }
        }
    }

    fn continue_token(&mut self, character: &char) {
        if self.token_inferred_kind.is_none() {
            return;
        }
        self.token_content.push(*character);

        let inferred_kind = self.token_inferred_kind.clone().unwrap();
        match inferred_kind {
            TokenKind::Identifier(_) => {
                if !TokenKind::is_valid_identifier_character(false, self.peek().get_or_insert('\0'))
                {
                    if let Some(boolean_literal) =
                        LiteralKind::into_boolean_literal(&self.token_content)
                    {
                        self.token_inferred_kind = Some(TokenKind::Literal(boolean_literal));
                        self.push_token();
                        return;
                    }

                    if let Some(keyword) = KeywordKind::create_keyword(&self.token_content) {
                        self.token_inferred_kind = Some(TokenKind::Keyword(keyword));
                    } else {
                        self.token_inferred_kind =
                            Some(TokenKind::Identifier(self.token_content.clone()));
                    }
                    self.push_token();
                }
            }
            // Int8 is used as the defacto unknown number marker for integers
            // Float32 is used as the defacto unknown number marker for floats
            TokenKind::Literal(LiteralKind::Int8(_))
            | TokenKind::Literal(LiteralKind::Float32(_)) => {
                if !TokenKind::is_valid_numeric_character(self.peek().get_or_insert('\0')) {
                    let numeric_literal =
                        match LiteralKind::into_numeric_literal(&self.token_content) {
                            Ok(x) => x,
                            Err(_err) => todo!("Do implement this error message"),
                        };
                    self.token_inferred_kind = Some(TokenKind::Literal(numeric_literal));
                    self.push_token();
                }
            }
            TokenKind::Literal(LiteralKind::Str(_)) => {
                let peek = match self.peek() {
                    Some(x) => x,
                    None => todo!("error here, unexpected end to a token"),
                };
                if peek == '"' && *character != '\\' {
                    self.token_inferred_kind = Some(TokenKind::Literal(
                        LiteralKind::into_string_literal(&self.token_content),
                    ));
                    self.push_token();
                    self.source.next(); // consume
                }
            }
            TokenKind::Literal(LiteralKind::Char(_)) => {
                let peek = match self.peek() {
                    Some(x) => x,
                    None => todo!("error here, unexpected end to a token"),
                };
                if peek == '\'' && *character != '\\' {
                    let character_literal =
                        match LiteralKind::into_char_literal(&self.token_content) {
                            Ok(x) => x,
                            Err(_err) => todo!("implement the error message here"),
                        };
                    self.token_inferred_kind = Some(TokenKind::Literal(character_literal));
                    self.push_token();
                    self.source.next(); // consume
                }
            }

            _ => {
                todo!("maybe error here? Although this branch is unreachable - i think")
            }
        }
    }
}
