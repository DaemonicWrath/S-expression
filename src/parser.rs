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

    pub fn parse(mut self) -> Parse {
        self.builder.start_node(SyntaxKind::Root.into());

        grammar::root(&mut self);

        self.builder.finish_node();

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    pub(crate) fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    pub(crate) fn expect(&mut self, kind: TokenKind, msg: String) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error(msg);
        }
    }

    pub(crate) fn expect_set(&mut self, set: &[TokenKind], msg: String) {
        if self.at_set(set) {
            self.bump();
        } else {
            self.error(msg);
        }
    }

    pub(crate) fn error(&mut self, msg: String) {
        self.builder.start_node(SyntaxKind::Error.into());
        self.errors.push(msg);
        self.bump(); // be sure to chug along in case of error
        self.builder.finish_node()
    }

    pub(crate) fn eat_ws(&mut self) {
        while self.peek() == Some(TokenKind::Whitespace) {
            self.tokens.pop();
        }
    }

    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == Some(kind)
    }

    pub(crate) fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn peek(&self) -> Option<TokenKind> {
        self.tokens.last().map(|token| token.kind)
    }

    pub(crate) fn bump(&mut self) {
        let token = self.tokens.pop().unwrap();
        self.builder
            .token(Lang::kind_to_raw(token.kind.into()), token.text.into());

        self.eat_ws();
    }
}
