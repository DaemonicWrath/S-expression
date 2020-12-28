use crate::{
    grammar,
    lexer::{Token, TokenKind},
    syntax::{Lang, SyntaxKind},
};
use rowan::{GreenNode, GreenNodeBuilder, Language};
#[derive(Debug)]
pub struct Parse {
    pub green_node: GreenNode,
    #[allow(unused)]
    pub errors: Vec<String>,
}
// Parser wraps the functionality of rowans GreenNodeBuilder providing useful utilities
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    // consumes self and generates a Parse result
    pub fn parse(mut self) -> Parse {
        grammar::root(&mut self);

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    // wrapper for GreenNodeBuilder start_node
    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    // wrapper for GreenNodeBuilder start_node
    pub(crate) fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    // checks if the current token is equal to some kind or adds an error
    pub(crate) fn expect(&mut self, kind: TokenKind, msg: String) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error(msg);
        }
    }

    // checks if the current token is one of a set of values or adds an error
    pub(crate) fn expect_set(&mut self, set: &[TokenKind], msg: String) {
        if self.at_set(set) {
            self.bump();
        } else {
            self.error(msg);
        }
    }

    // wraps the current token in a error and adds it to a list
    pub(crate) fn error(&mut self, msg: String) {
        self.builder.start_node(SyntaxKind::Error.into());
        self.errors.push(msg);
        self.bump(); // be sure to chug along in case of error
        self.builder.finish_node()
    }

    // removes whitespace until we are at a token
    pub(crate) fn eat_ws(&mut self) {
        while self.peek() == Some(TokenKind::Whitespace) {
            self.tokens.pop();
        }
    }

    // checks if the current token is some given kind
    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == Some(kind)
    }

    // checks if the current token is in some set of kinds
    pub(crate) fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    // gets the current token kind without removing it
    pub(crate) fn peek(&self) -> Option<TokenKind> {
        self.tokens.last().map(|token| token.kind)
    }

    // removes the token from the list and adds it to rowan list of tokens
    pub(crate) fn bump(&mut self) {
        let token = self.tokens.pop().unwrap();
        self.builder
            .token(Lang::kind_to_raw(token.kind.into()), token.text.into());

        self.eat_ws();
    }
}
