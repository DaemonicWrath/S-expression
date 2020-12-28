use crate::{lexer::TokenKind, parser::Parser, syntax::SyntaxKind};

// creates the root node and generates the syntax tree
pub(crate) fn root(parser: &mut Parser) {
    parser.start_node(SyntaxKind::Root.into());

    if parser.at(TokenKind::LParen) {
        list(parser);
    } else if parser.at(TokenKind::Number) {
        literal(parser);
    } else {
        parser.error("root should start with a 'Number' or '('".to_string());
    }

    parser.finish_node();
}

// handles parsing for a list expression
// Example: (add 1 2)
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

// handles parsing numeric literals
fn literal(parser: &mut Parser) {
    assert!(parser.at(TokenKind::Number));
    parser.start_node(SyntaxKind::Literal);
    parser.bump();
    parser.finish_node();
}
