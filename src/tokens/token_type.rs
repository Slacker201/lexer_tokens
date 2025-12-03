

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Punctuation(Punctuation),
    Delimiter,
    Identifier(String),
    Keyword,
    Symbol(Symbol),
    Unknown(String),
    String(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Punctuation {
    Period,
    SemiColon,
    Exclamation,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    At,
    Equals,
    EqualityComparison,
    Hyphen,
    Plus,
    ForwardSlash,
    Astrix,
    Ampersand,
    Hashtag,
    Percent,
    Caret,
    Question,
    OpenAngle,
    CloseAngle,
}