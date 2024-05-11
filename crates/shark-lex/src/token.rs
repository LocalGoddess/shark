use std::ops::RangeInclusive;

use shark_core::source::SourcePosition;

/// Represents a token produced during lexical analysis. [LexerToken]s give more meaning to the
/// source code because each token resembles are certain concept in the language such as a keyword,
/// identifier, operator, and others.
#[derive(Debug, Clone)]
pub struct LexerToken<'token> {
    pub kind: TokenKind,

    pub position: RangeInclusive<SourcePosition<'token>>,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifer(String),
    Keyword(KeywordKind),
    Literal(LiteralKind),

    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /

    PlusAssign,     // +=
    MinusAssign,    // -=
    MultiplyAssign, // *-
    DivideAssign,   // /=

    Greater, // >
    Lesser,  // <
    Or,      // |
    Not,     // !
    And,     // &&
    Equal,   // =

    GreaterOrEqual, // >=
    LessOrEqual,    // <=
    NotEqual,       // !=
    EqualTo,        // ==

    ShiftRight, // >>
    ShiftLeft,  // <<
    BitwiseAnd, // &

    Comma,      // ,
    TypeAssign, // ::
    Dot,        // .
    CurlyBrace { opened: bool },
    Parenthesis { opened: bool },

    EOL, // ; and potentially newline
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
    Of,
    Ptr,
    Pub,
    Ref,
    Ret,
    Trait,
    Type,
    Unsafe,
    Use,
    When,
    Where,
    Yield,
}

impl KeywordKind {
    pub fn is_keyword(identifier: String) -> Option<Self> {
        match identifier.as_str() {
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "for" => Some(Self::For),
            "fun" => Some(Self::Fun),
            "if" => Some(Self::If),
            "in" => Some(Self::In),
            "let" => Some(Self::Let),
            "mut" => Some(Self::Mut),
            "of" => Some(Self::Of),
            "ptr" => Some(Self::Ptr),
            "pub" => Some(Self::Pub),
            "ref" => Some(Self::Ref),
            "ret" => Some(Self::Ret),
            "trait" => Some(Self::Trait),
            "type" => Some(Self::Type),
            "unsafe" => Some(Self::Unsafe),
            "use" => Some(Self::Use),
            "when" => Some(Self::When),
            "where" => Some(Self::Where),
            "yield" => Some(Self::Yield),

            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    // Integer Literals
    UInt8(u8),
    Int8(i8),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),

    // Floating Point Literals
    Float32(f32),
    Float64(f64),

    // Array-like Literals
    Str(String),
    // Array(String), TODO(Chloe): Figure This out
    // Array literals are just [ ... ] where ... is a comma separated list of values
    Char(char),
    Boolean(bool),
}

// TODO(Chloe): Create functions for identifying and converting values into the proper type
