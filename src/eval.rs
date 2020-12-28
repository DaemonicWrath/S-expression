use crate::{
    parser::Parse,
    syntax::{SyntaxKind, SyntaxNode},
};

enum Operation {
    Add,
    Multiply,
}

// Given a valid parse return the result
pub fn eval(parse: Parse) -> i64 {
    let root = SyntaxNode::new_root(parse.green_node.clone());

    let first_child = root.first_child().unwrap(); // have to get the first child as the root is the SyntaxKind::Root type

    match first_child.kind() {
        SyntaxKind::List => eval_list(&first_child),
        SyntaxKind::Literal => eval_literal(&first_child),
        _ => unreachable!(),
    }
}

pub fn eval_literal(node: &SyntaxNode) -> i64 {
    match node.green().children().next() {
        Some(rowan::NodeOrToken::Token(token)) => token.text().parse().unwrap(),
        _ => unreachable!(),
    }
}

pub fn eval_list(root: &SyntaxNode) -> i64 {
    let mut iter = root.children_with_tokens(); // we need tokens as we don't wrap operations like 'add' or 'multiply' in a abstract type

    assert_eq!(iter.next().unwrap().kind(), SyntaxKind::LParen); // first child is always a '('

    let op = as_operation(&iter.next().unwrap().kind()).unwrap();

    let arg1 = parse_arg(&iter.next().unwrap().as_node().unwrap());
    let arg2 = parse_arg(&iter.next().unwrap().as_node().unwrap());
    match op {
        Operation::Add => arg1 + arg2,
        Operation::Multiply => arg1 * arg2,
    }
}

fn parse_arg(node: &SyntaxNode) -> i64 {
    assert!(node.kind() == SyntaxKind::List || node.kind() == SyntaxKind::Literal);
    match node.kind() {
        SyntaxKind::List => eval_list(node),
        SyntaxKind::Literal => eval_literal(node),
        _ => unreachable!(),
    }
}

fn as_operation(kind: &SyntaxKind) -> Option<Operation> {
    match kind {
        SyntaxKind::Add => Some(Operation::Add),
        SyntaxKind::Multiply => Some(Operation::Multiply),
        _ => None,
    }
}
