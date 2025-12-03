

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Punctuation(Punctuation),
    Delimiter,
    Identifier,
    Keyword,
    Unknown(String),
    String(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Punctuation {
    Period,
    SemiColon,
    Hyphen,
    Plus,
    Astrix,
    Ampersand,
    Exclamation,
    Equals,
    EqualityComparison,
}