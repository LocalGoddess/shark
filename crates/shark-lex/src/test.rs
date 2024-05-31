use crate::{
    token::TokenKind,
    token::{KeywordKind, LexerToken, LiteralKind},
    Lexer,
};

fn verify_tokens(returned_tokens: &Vec<LexerToken>, expected_tokens: &Vec<TokenKind>) -> bool {
    if returned_tokens.len() != expected_tokens.len() {
        return false;
    }
    for (index, token) in returned_tokens.iter().enumerate() {
        let expected = expected_tokens.get(index).unwrap(); // These are the same length so its
                                                            // fine
        if token.kind != *expected {
            return false;
        }
    }
    return true;
}

#[test]
fn test_identifier() {
    let mut lexer = Lexer::new(None, "this_is_a_crazy_identifier8080");
    lexer.lex();

    let expected = vec![TokenKind::Identifier(
        "this_is_a_crazy_identifier8080".to_string(),
    )];
    assert!(verify_tokens(&lexer.completed_tokens, &expected))
}

#[test]
fn test_keyword() {
    let mut lexer = Lexer::new(None, "fun");
    lexer.lex();

    let expected_tokens = vec![TokenKind::Keyword(KeywordKind::Fun)];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_literal_str() {
    let mut lexer = Lexer::new(None, "\"Hello, World\"");
    lexer.lex();

    let expected_tokens = vec![TokenKind::Literal(LiteralKind::Str(
        "Hello, World".to_string(),
    ))];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_literal_char() {
    let mut lexer = Lexer::new(None, "'h'");
    lexer.lex();

    let expected_tokens = vec![TokenKind::Literal(LiteralKind::Char('h'))];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_escapes() {
    let mut lexer = Lexer::new(None, "\"\\n \\t \\\\ \\u{263A}\"");
    lexer.lex();

    let expected_tokens = vec![TokenKind::Literal(LiteralKind::Str(
        "\n \t \\ \u{263A}".to_string(),
    ))];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_literal_bool() {
    let mut lexer = Lexer::new(None, "false");
    lexer.lex();

    let expected_tokens = vec![TokenKind::Literal(LiteralKind::Boolean(false))];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_literal_numerics() {
    let mut lexer = Lexer::new(None, "-1337 1337 -3.14 3.14 132uint8");
    lexer.lex();

    let expected_tokens = vec![
        TokenKind::Literal(LiteralKind::Int32(-1337)),
        TokenKind::Literal(LiteralKind::Int32(1337)),
        TokenKind::Literal(LiteralKind::Float32(-3.14)),
        TokenKind::Literal(LiteralKind::Float32(3.14)),
        TokenKind::Literal(LiteralKind::UInt8(132)),
    ];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_grammar() {
    let mut lexer = Lexer::new(None, "; - -= ::");
    lexer.lex();

    let expected_tokens = vec![
        TokenKind::EOL,
        TokenKind::Minus,
        TokenKind::MinusAssign,
        TokenKind::TypeAssign,
    ];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_big() {
    let mut lexer = Lexer::new(None, "pub fun main() {\n    let a :: Float32 = 3.14;\n}");
    lexer.lex();

    let expected_tokens = vec![
        TokenKind::Keyword(KeywordKind::Pub),
        TokenKind::Keyword(KeywordKind::Fun),
        TokenKind::Identifier(String::from("main")),
        TokenKind::Parenthesis { opened: true },
        TokenKind::Parenthesis { opened: false },
        TokenKind::CurlyBrace { opened: true },
        TokenKind::Keyword(KeywordKind::Let),
        TokenKind::Identifier(String::from("a")),
        TokenKind::TypeAssign,
        TokenKind::Identifier(String::from("Float32")),
        TokenKind::Equal,
        TokenKind::Literal(LiteralKind::Float32(3.14)),
        TokenKind::EOL,
        TokenKind::CurlyBrace { opened: false },
    ];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_condensed() {
    let mut lexer = Lexer::new(None, "1+1");
    lexer.lex();

    let expected_tokens = vec![
        TokenKind::Literal(LiteralKind::Int32(1)),
        TokenKind::Plus,
        TokenKind::Literal(LiteralKind::Int32(1)),
    ];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_comment() {
    let mut lexer = Lexer::new(None, "1// hello \n+// hello\n1");
    lexer.lex();

    let expected_tokens = vec![
        TokenKind::Literal(LiteralKind::Int32(1)),
        TokenKind::Plus,
        TokenKind::Literal(LiteralKind::Int32(1)),
    ];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}

#[test]
fn test_multiline_comment() {
    let mut lexer = Lexer::new(None, "1/* hello */+/* hello */1");
    lexer.lex();

    let expected_tokens = vec![
        TokenKind::Literal(LiteralKind::Int32(1)),
        TokenKind::Plus,
        TokenKind::Literal(LiteralKind::Int32(1)),
    ];
    assert!(verify_tokens(&lexer.completed_tokens, &expected_tokens));
}
