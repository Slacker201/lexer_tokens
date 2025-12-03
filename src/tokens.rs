use crate::tokens::{span::Span, token_type::TokenType};


pub mod span;
pub mod token_type;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub span: Span,
    pub token_type: TokenType,
}

