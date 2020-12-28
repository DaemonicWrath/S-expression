use crate::lexer::TokenKind;
// a more detailed version of TOkenKind that contains abstract syntax types
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum SyntaxKind {
    Whitespace,
    Number,
    Add,
    Multiply,
    LParen,
    RParen,
    Error,

    List,
    Literal,
    Root,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token: TokenKind) -> Self {
        match token {
            TokenKind::Whitespace => SyntaxKind::Whitespace,
            TokenKind::Number => SyntaxKind::Number,
            TokenKind::Add => SyntaxKind::Add,
            TokenKind::Multiply => SyntaxKind::Multiply,
            TokenKind::LParen => SyntaxKind::LParen,
            TokenKind::RParen => SyntaxKind::RParen,
            TokenKind::Error => SyntaxKind::Error,
        }
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Lang {}

impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Root as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}
