use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum NumberTy {
    U32,
    Unspecified,
}

#[derive(Debug, PartialEq)]
pub enum LiteralTy {
    Number(NumberTy),
}

pub const SYMBOLS: [&str; 6] = ["+=", "+", "(", ")", "{", "}"];

#[derive(Debug, PartialEq)]
pub enum SymbolTy {
    Add,
    AddAssign,
    OpParen,
    ClParen,
    OpBrace,
    ClBrace,
}

pub const KEYWORDS: [&str; 6] = ["fn", "while", "for", "map", "validator", "validate"];

#[derive(Debug, PartialEq)]
pub enum KeywordTy {
    Function, // fn
}

#[derive(Debug, PartialEq)]
pub enum TokenTy {
    Literal(LiteralTy),
    Symbol(SymbolTy),
    Keyword(KeywordTy),
    Identifier,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub range: Range<usize>,
    pub ty: TokenTy,
}
