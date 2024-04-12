pub mod lexer;

#[derive(Debug, Clone)]
pub struct LexerToken {
    pub kind: TokenKind,

    pub row: usize,
    pub column: usize,
    pub length: usize,
}

impl LexerToken {
    pub fn new(kind: TokenKind, row: usize, column: usize, length: usize) -> Self {
        Self {
            kind,
            row,
            column,
            length,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum KeywordKind {
    Else,
    Enum,
    For,
    Fun,
    If,
    Let,
    Mut,
    Pub,
    Ref,
    Ret,
    Trait,
    Type,
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
            "let" => Some(Self::Let),
            "mut" => Some(Self::Mut),
            "pub" => Some(Self::Pub),
            "ref" => Some(Self::Ref),
            "ret" => Some(Self::Ret),
            "trait" => Some(Self::Trait),
            "type" => Some(Self::Type),
            "when" => Some(Self::When),
            "yield" => Some(Self::Yield),

            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LiteralKind<'a> {
    Str(&'a str),
    Int(i32),
    Boolean(bool),
    Float(f32),
    Char(&'a str),
}

impl<'a> LiteralKind<'a> {
    pub fn bool_from_string(string: &str) -> Option<Self> {
        match string {
            "true" => Some(LiteralKind::Boolean(true)),
            "false" => Some(LiteralKind::Boolean(false)),

            _ => None,
        }
    }
}
