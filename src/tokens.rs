use crate::tokens::{span::Span, token_type::TokenType};


pub mod span;
pub mod token_type;


pub struct Token {
    span: Span,
    token_type: TokenType,
}

