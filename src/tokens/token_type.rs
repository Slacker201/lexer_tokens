

pub enum TokenType {
    Punctuation(Punctuation),
    Delimiter,
    Identifier,
    Keyword,
    Unknown(String),
}

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