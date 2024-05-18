use std::{error::Error, ops::RangeInclusive};

use shark_core::source::SourcePosition;

use crate::error::{
    InvalidCharacterLiteralErrrorKind, InvalidCharacterLiteralSizeError, InvalidFloatRadix,
    UnknownNumericSuffixError,
};

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

impl TokenKind {
    /// Attempts to create a grammar token using the current character and the next character
    pub fn create_grammar_token(current: &char, peek: Option<&char>) -> Option<TokenKind> {
        match current {
            '+' => match peek {
                Some('=') => Some(TokenKind::PlusAssign),
                _ => Some(TokenKind::Plus),
            },
            '-' => match peek {
                Some('=') => Some(TokenKind::MinusAssign),
                _ => Some(TokenKind::Minus),
            },
            '*' => match peek {
                Some('=') => Some(TokenKind::MultiplyAssign),
                _ => Some(TokenKind::Multiply),
            },
            '/' => match peek {
                Some('=') => Some(TokenKind::DivideAssign),
                _ => Some(TokenKind::Divide),
            },
            '>' => match peek {
                Some('=') => Some(TokenKind::GreaterOrEqual),
                Some('>') => Some(TokenKind::ShiftRight),
                _ => Some(TokenKind::Greater),
            },
            '<' => match peek {
                Some('=') => Some(TokenKind::LessOrEqual),
                Some('<') => Some(TokenKind::ShiftLeft),
                _ => Some(TokenKind::Lesser),
            },
            '!' => match peek {
                Some('=') => Some(TokenKind::NotEqual),
                _ => Some(TokenKind::Not),
            },
            '=' => match peek {
                Some('=') => Some(TokenKind::EqualTo),
                _ => Some(TokenKind::Equal),
            },
            '&' => match peek {
                Some('&') => Some(TokenKind::And),
                _ => Some(TokenKind::BitwiseAnd),
            },
            ':' => match peek {
                Some(':') => Some(TokenKind::TypeAssign),
                _ => None,
            },
            '|' => Some(TokenKind::Or),
            ',' => Some(TokenKind::Comma),
            '.' => Some(TokenKind::Dot),
            '{' => Some(TokenKind::CurlyBrace { opened: true }),
            '}' => Some(TokenKind::CurlyBrace { opened: false }),
            '(' => Some(TokenKind::Parenthesis { opened: true }),
            ')' => Some(TokenKind::Parenthesis { opened: false }),
            ';' => Some(TokenKind::EOL),
            _ => None,
        }
    }
    
    /// Gets the length of a grammar token for use in calculating how many characters to consume
    /// after using [TokenKind::create_grammar_token]
    pub(crate) fn get_grammar_token_length(&self) -> usize {
        match self {
            Self::PlusAssign
            | Self::MinusAssign
            | Self::MultiplyAssign
            | Self::DivideAssign
            | Self::GreaterOrEqual
            | Self::ShiftRight
            | Self::LessOrEqual
            | Self::ShiftLeft
            | Self::NotEqual
            | Self::EqualTo
            | Self::And
            | Self::TypeAssign => 2,
            _ => 1,
        }
    }
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
    /// Attempts to create a keyword based on the string inputted
    pub fn create_keyword(identifier: String) -> Option<Self> {
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

impl LiteralKind {
    /// Reads a literal integer in order to get the radix (base) of it
    fn get_literal_integer_radix(working_content: &str) -> (u32, bool) {
        let is_negative = working_content.starts_with('-');
        let operating_content = if is_negative {
            &working_content[1..]
        } else {
            working_content
        };

        let prefix = &operating_content[..2.min(operating_content.len())].to_lowercase();
        let radix = match prefix.as_str() {
            "0x" => 16,
            "0o" => 8,
            "0b" => 2,
            _ => 10,
        };

        (radix, is_negative)
    }

    /// Converts a token's working_content to a numeric [LiteralKind]
    /// If this function is supplied something other than a number, it will probably break so its
    /// up to the caller to make sure the incoming content is a number.
    pub fn into_numeric_literal(working_content: &str) -> Result<LiteralKind, Box<dyn Error>> {
        let (radix, is_negative) = Self::get_literal_integer_radix(working_content);
        let actual_number = if radix != 10 {
            let mut result = working_content[if is_negative { 3 } else { 2 }..].to_owned(); // remove this 0x, 0b, or 0o

            // readd the previously removed negative sign
            if is_negative {
                result = format!("-{}", result);
            }
            result
        } else {
            working_content.to_owned()
        };

        for (index, c) in actual_number.chars().enumerate() {
            if !c.is_ascii_digit() && c != '-' && c != '.' {
                let suffix = &actual_number[index..];
                let numeric_part = &actual_number[0..index];

                return match suffix {
                    "uint8" => Ok(LiteralKind::UInt8(u8::from_str_radix(numeric_part, radix)?)),
                    "int8" => Ok(LiteralKind::Int8(i8::from_str_radix(numeric_part, radix)?)),
                    "uint32" => Ok(LiteralKind::UInt32(u32::from_str_radix(
                        numeric_part,
                        radix,
                    )?)),
                    "int32" => Ok(LiteralKind::Int32(i32::from_str_radix(
                        numeric_part,
                        radix,
                    )?)),
                    "uint64" => Ok(LiteralKind::UInt64(u64::from_str_radix(
                        numeric_part,
                        radix,
                    )?)),
                    "int64" => Ok(LiteralKind::Int64(i64::from_str_radix(
                        numeric_part,
                        radix,
                    )?)),

                    // Only allow base 10 in float literals
                    "float32" => {
                        if radix != 10 {
                            Err(Box::new(InvalidFloatRadix))
                        } else {
                            Ok(LiteralKind::Float32(numeric_part.parse::<f32>()?))
                        }
                    }
                    "float64" => {
                        if radix != 10 {
                            Err(Box::new(InvalidFloatRadix))
                        } else {
                            Ok(LiteralKind::Float64(numeric_part.parse::<f64>()?))
                        }
                    }
                    _ => Err(Box::new(UnknownNumericSuffixError {
                        invalid_suffix: suffix.to_string(),
                    })),
                };
            }
        }

        // If no suffix is specified, just default to a Float32 for decimal numbers and Int32 for
        // whole numbers
        if actual_number.contains('.') {
            Ok(LiteralKind::Float32(working_content.parse::<f32>()?))
        } else {
            Ok(LiteralKind::Int32(working_content.parse::<i32>()?))
        }
    }

    /// Converts a token's working_content into a [LiteralKind::Char]
    /// This function assumes the provided content is somewhere near a character
    pub fn into_char_literal(working_content: &str) -> Result<LiteralKind, Box<dyn Error>> {
        let mut value = working_content.to_string();

        // Remove surrounding '
        if value.starts_with('\'') {
            value = working_content[1..].to_string();
        }
        if value.ends_with('\'') {
            value = working_content[..value.len()].to_string();
        }

        value = unescape_characters!(value);
        if value.len() > 1 {
            return Err(Box::new(InvalidCharacterLiteralSizeError {
                kind: InvalidCharacterLiteralErrrorKind::TooLong,
            }));
        }

        let character: char = match value.chars().next() {
            Some(x) => x,
            None => {
                return Err(Box::new(InvalidCharacterLiteralSizeError {
                    kind: InvalidCharacterLiteralErrrorKind::Empty,
                }))
            }
        };
        Ok(LiteralKind::Char(character))
    }

    /// Converts a token's working_content into a [LiteralKind::Str]
    /// This function assumes the provided content is somewhere near a character       
    pub fn into_string_literal(working_content: &str) -> LiteralKind {
        let mut value = working_content.to_string();

        // Trim off the starting and ending quotations if they exist
        if value.starts_with('"') {
            value = value[1..].to_string();
        }
        if value.ends_with('"') {
            value = value[..value.len()].to_string();
        }

        let value = unescape_characters!(value);
        LiteralKind::Str(value)
    }

    // Most useful function ever might remove it
    pub fn into_boolean_literal(working_content: &str) -> Option<LiteralKind> {
        match working_content {
            "true" => Some(LiteralKind::Boolean(true)),
            "false" => Some(LiteralKind::Boolean(false)),
            _ => None,
        }
    }
}
