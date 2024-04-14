use crate::{lexer::Lexer, LexerToken, TokenKind};

/// For use when testing single tokens to make sure they are what they are suppose to be
fn check_correct_token(tokens: &Vec<LexerToken>, expected_kind: TokenKind) -> bool {
    if tokens.len() != 1 {
        return false;
    }
    tokens.get(0).unwrap().kind == expected_kind
}

fn check_correct_token_order(tokens: &Vec<LexerToken>, expected: &Vec<TokenKind>) -> bool {
    for (index, token) in tokens.iter().enumerate() {
        if let Some(expected_token) = expected.get(index) {
            if token.kind != *expected_token {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn _debug_token(tokens: &Vec<LexerToken>, position: usize) {
    if let Some(token) = tokens.get(position) {
        dbg!(token);
    } else {
        println!("Token at {} doesn't exist", position);
    }
}

#[test]
fn test_identifier() {
    let mut lexer = Lexer::new("this_is_a_crazy_identifier111");
    let tokens = lexer.lex();
    assert!(check_correct_token(
        &tokens,
        TokenKind::Identifier("this_is_a_crazy_identifier111".to_owned())
    ))
}

#[test]
fn test_keyword() {
    let mut lexer = Lexer::new("fun");
    let tokens = lexer.lex();
    assert!(check_correct_token(
        &tokens,
        TokenKind::Keyword(crate::KeywordKind::Fun)
    ))
}

#[test]
fn test_literal_str() {
    let mut lexer = Lexer::new("\"Hello, World\"");
    let tokens = lexer.lex();
    assert!(check_correct_token(
        &tokens,
        TokenKind::Literal(crate::LiteralKind::Str("Hello, World".to_owned()))
    ));
}

#[test]
fn test_literal_char() {
    let mut lexer = Lexer::new("'h'");
    let tokens = lexer.lex();
    assert!(check_correct_token(
        &tokens,
        TokenKind::Literal(crate::LiteralKind::Char("h".to_owned()))
    )); // Holy shit the parenths
}

#[test]
fn test_literal_bool() {
    let mut lexer = Lexer::new("true");
    let tokens = lexer.lex();
    assert!(check_correct_token(
        &tokens,
        TokenKind::Literal(crate::LiteralKind::Boolean(true))
    ));
}

#[test]
fn test_literal_int32() {
    let mut lexer = Lexer::new("-1337");
    let tokens = lexer.lex();
    assert!(check_correct_token(
        &tokens,
        TokenKind::Literal(crate::LiteralKind::Int(-1337))
    ));
}

#[test]
fn test_literal_float32() {
    let mut lexer = Lexer::new("-3.14");
    let tokens = lexer.lex();
    assert!(check_correct_token(
        &tokens,
        TokenKind::Literal(crate::LiteralKind::Float(-3.14))
    ));
}

#[test]
fn test_single_char_token() {
    let mut lexer = Lexer::new(";");
    let tokens = lexer.lex();
    assert!(check_correct_token(&tokens, TokenKind::EOL));
}

#[test]
fn test_big() {
    let mut lexer = Lexer::new("pub fun main() {\n    let a :: Float32 = 3.14;\n}");
    let tokens = lexer.lex();

    let expected = vec![
        TokenKind::Keyword(crate::KeywordKind::Pub),
        TokenKind::Keyword(crate::KeywordKind::Fun),
        TokenKind::Identifier(String::from("main")),
        TokenKind::Parenthesis { opened: true },
        TokenKind::Parenthesis { opened: false },
        TokenKind::CurlyBrace { opened: true },
        TokenKind::Keyword(crate::KeywordKind::Let),
        TokenKind::Identifier(String::from("a")),
        TokenKind::Colon,
        TokenKind::Colon,
        TokenKind::Identifier(String::from("Float32")),
        TokenKind::Equal,
        TokenKind::Literal(crate::LiteralKind::Float(3.14)),
        TokenKind::EOL,
        TokenKind::CurlyBrace { opened: false },
    ];

    assert!(check_correct_token_order(&tokens, &expected));
}

#[test]
fn test_condensed() {
    let mut lexer = Lexer::new("1+1");
    let tokens = lexer.lex();

    let expected = vec![
        TokenKind::Literal(crate::LiteralKind::Int(1)),
        TokenKind::Plus,
        TokenKind::Literal(crate::LiteralKind::Int(1)),
    ];

    assert!(check_correct_token_order(&tokens, &expected));
}
