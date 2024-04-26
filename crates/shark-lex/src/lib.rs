use shark_error::source::SourcePosition;

pub mod lexer;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct LexerToken<'a> {
    pub kind: TokenKind,

    pub position: SourcePosition<'a>,
    pub length: usize,
}

impl<'a> LexerToken<'a> {
    pub fn new(kind: TokenKind, position: SourcePosition<'a>, length: usize) -> Self {
        Self {
            kind,
            position,
            length,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Keyword(KeywordKind),
    Literal(LiteralKind),

    Plus,                           // +
    Minus,                          // -
    Equal,                          // =
    Astrisk,                        // *
    Slash,                          // /
    Percent,                        // %
    Ampersand,                      // &
    Pipe,                           // |
    Bang,                           // !
    Caret,                          // ^
    Dot,                            // .
    Colon,                          // :
    Apostrophe,                     // '
    Comma,                          // ,
    UnderScore,                     // _
    AngleBracket { opened: bool },  // < >
    CurlyBrace { opened: bool },    // { }
    SquareBracket { opened: bool }, // [ ]
    Parenthesis { opened: bool },   // ( )

    EOL, // ;
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    Else,
    Enum,
    For,
    Fun,
    If,
    In,
    Let,
    Mut,
    Ptr,
    Pub,
    Ref,
    Ret,
    Trait,
    Type,
    Unsafe,
    When,
    Yield,
}

impl KeywordKind {
    pub fn is_keyword(identifier: &str) -> Option<Self> {
        match identifier {
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "for" => Some(Self::For),
            "fun" => Some(Self::Fun),
            "if" => Some(Self::If),
            "in" => Some(Self::In),
            "let" => Some(Self::Let),
            "mut" => Some(Self::Mut),
            "ptr" => Some(Self::Ptr),
            "pub" => Some(Self::Pub),
            "ref" => Some(Self::Ref),
            "ret" => Some(Self::Ret),
            "trait" => Some(Self::Trait),
            "type" => Some(Self::Type),
            "unsafe" => Some(Self::Unsafe),
            "when" => Some(Self::When),
            "yield" => Some(Self::Yield),

            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Str(String),
    Int(i32),
    Boolean(bool),
    Float(f32),
    Char(String),
}

impl LiteralKind {
    pub fn bool_from_string(string: &str) -> Option<Self> {
        match string {
            "true" => Some(LiteralKind::Boolean(true)),
            "false" => Some(LiteralKind::Boolean(false)),

            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Comment {
    LineComment,
    BlockComment,
}
