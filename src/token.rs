use lazy_static::lazy_static;
use std::{collections::HashMap, ops::Range};

#[derive(Debug, PartialEq)]
pub enum NumberTy {
    U32,
    Unspecified,
}

#[derive(Debug, PartialEq)]
pub enum LiteralTy {
    Number(NumberTy),
}

lazy_static! {
    pub static ref SYMBOLS: HashMap<&'static str, SymbolTy> = {
        let mut map = HashMap::new();
        map.insert("->", SymbolTy::ThinArrow);
        map.insert("+=", SymbolTy::AddAssign);
        map.insert("=", SymbolTy::Assign);
        map.insert("+", SymbolTy::Add);
        map.insert(">", SymbolTy::Greater);
        map.insert("(", SymbolTy::OpParen);
        map.insert(")", SymbolTy::ClParen);
        map.insert("{", SymbolTy::OpBrace);
        map.insert("}", SymbolTy::ClBrace);
        map.insert(";", SymbolTy::EndStmt);
        map
    };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymbolTy {
    Assign,
    Add,
    AddAssign,
    Greater,
    OpParen,
    ClParen,
    OpBrace,
    ClBrace,
    EndStmt,
    ThinArrow,
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, KeywordTy> = {
        let mut map = HashMap::new();
        map.insert("fn", KeywordTy::Function);
        map.insert("let", KeywordTy::Let);
        map.insert("if", KeywordTy::If);
        map.insert("return", KeywordTy::Return);
        map
    };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeywordTy {
    Function, // fn
    Let,      // let
    If,       // if
    Return,
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
