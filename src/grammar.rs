use crate::{lexer::TokenKind, parser::Parser, syntax::SyntaxKind};

pub(crate) fn root(parser: &mut Parser) {
    if parser.at(TokenKind::LParen) {
        list(parser);
    } else if parser.at(TokenKind::Number) {
        parser.start_node(SyntaxKind::Literal);
        parser.bump();
        parser.finish_node();
    } else {
        parser.error("root should start with a \"Number\" or \"(\"".to_string());
    }
}

fn list(parser: &mut Parser) {
    assert!(parser.at(TokenKind::LParen));
    parser.start_node(SyntaxKind::List);

    parser.bump();

    parser.expect_set(
        &[TokenKind::Add, TokenKind::Multiply],
        "expected 'add', or 'multiply'".to_string(),
    );

    for _ in 0..2 {
        match parser.peek() {
            Some(TokenKind::LParen) => list(parser),
            Some(TokenKind::Number) => literal(parser),
            _ => parser.error("Expected '(' or number".to_string()),
        }
    }

    parser.expect(TokenKind::RParen, "expected )".to_string());

    parser.finish_node();
}

fn literal(parser: &mut Parser) {
    parser.start_node(SyntaxKind::Literal);
    parser.bump();
    parser.finish_node();
}
