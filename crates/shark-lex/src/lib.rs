#[macro_use]
pub mod macros;

use std::{path::Path, str::Chars};

use shark_core::source::SourcePosition;
use token::{LexerToken, TokenKind};

pub mod error;
pub mod token;

#[derive(Debug)]
pub struct Lexer<'lexer> {
    // Basic Lexer State
    pub source: Chars<'lexer>,
    pub source_length: usize,
    pub current_position: SourcePosition<'lexer>,
    pub completed_tokens: Vec<LexerToken<'lexer>>,

    // Current Token State
    pub token_start_position: Option<SourcePosition<'lexer>>,
    pub token_content: String,
    pub token_inferred_kind: Option<TokenKind>,
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
        }
    }

    pub fn push_token(&mut self) {
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

    pub fn reset_token_state(&mut self) {
        self.token_start_position = None;
        self.token_inferred_kind = None;
        self.token_content = String::new();
    }
}
