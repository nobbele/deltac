use std::ops::Range;

pub mod ast;
pub mod lexer;

#[derive(Debug, PartialEq)]
pub struct TextSection {
    pub index: Range<usize>,
    pub line: Range<usize>,
    pub column: Range<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PrimitiveTy {
    Number,
}
