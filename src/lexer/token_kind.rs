use std::fmt;

use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum TokenKind {
    #[regex("[ \n]+")]
    Whitespace,

    #[regex("[0-9]+")]
    Number,

    #[token("add")]
    Add,

    #[token("multiply")]
    Multiply,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[error]
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::Number => "number",
            Self::Add => "‘add’",
            Self::Multiply => "‘multiply’",
            Self::LParen => "‘(’",
            Self::RParen => "‘)’",
            Self::Error => "an unrecognized token",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = TokenKind::lexer(input);

        let kind_result = lexer.next().unwrap();
        let text_result = lexer.slice();
        assert_eq!(kind_result, kind);
        assert_eq!(text_result, input);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_number() {
        check("123456", TokenKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("add", TokenKind::Add);
    }

    #[test]
    fn lex_multiply() {
        check("multiply", TokenKind::Multiply);
    }

    #[test]
    #[test]
    fn lex_left_parenthesis() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", TokenKind::RParen);
    }
}
