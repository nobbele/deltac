use itertools::Itertools;
use std::ops::Range;

use crate::{PrimitiveTy, TextSection};

use super::token::{KeywordTy, SymbolTy, Token, TokenTy, KEYWORDS, SYMBOLS};

enum TokenResult {
    Token(Token),
    Skip,
    InvalidCharacter(char),
    End,
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    pub remaining: &'a str,
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &str) -> Tokenizer {
        Tokenizer {
            remaining: s,
            line: 0,
            column: 0,
            index: 0,
        }
    }

    fn read_number(&mut self) -> Range<usize> {
        let length = self
            .remaining
            .chars()
            .peeking_take_while(|c| c.is_digit(10))
            .count();
        let range = self.index..self.index + length;
        self.increment_col(length);
        self.remaining = &self.remaining[length..];
        range
    }

    fn read_symbol(&mut self) -> (SymbolTy, Range<usize>) {
        let start_index = self.index;
        let (symbols_str, ty) = SYMBOLS
            .iter()
            .find(|symbol| self.remaining.starts_with(symbol.0))
            .expect("Not a proper symbol");
        let symbol_length = symbols_str.len();
        self.increment_char(symbol_length);
        self.increment_col(symbol_length);
        (*ty, start_index..self.index)
    }

    fn read_keyword(&mut self) -> (KeywordTy, Range<usize>) {
        let start_index = self.index;
        let (keyword_str, ty) = KEYWORDS
            .iter()
            .find(|keyword| self.remaining.starts_with(keyword.0))
            .expect("Not a proper keyword");
        let keyword_length = keyword_str.len();
        self.increment_char(keyword_length);
        self.increment_col(keyword_length);
        (*ty, start_index..self.index)
    }

    fn increment_col(&mut self, n: usize) {
        self.column += n;
        self.index += n;
    }

    fn increment_line(&mut self) {
        self.column = 0;
        self.line += 1;
        self.index += 1;
    }

    fn increment_char(&mut self, n: usize) {
        let next_index = match self.remaining.char_indices().nth(n) {
            Some(o) => o.0,
            None => self.remaining.len(),
        };
        self.remaining = &self.remaining[next_index..];
    }

    fn next_token(&mut self) -> TokenResult {
        let c = match self.remaining.chars().nth(0) {
            Some(o) => o,
            None => return TokenResult::End,
        };
        if c == ' ' || c == '\r' {
            self.remaining = &self.remaining[1..];
            self.increment_col(1);
            return TokenResult::Skip;
        }

        if c == '\n' {
            self.remaining = &self.remaining[1..];
            self.increment_line();
            return TokenResult::Skip;
        }

        if c.is_digit(10) {
            let number = self.read_number();
            return TokenResult::Token(Token {
                range: number,
                ty: TokenTy::Literal(PrimitiveTy::Number),
            });
        }

        if SYMBOLS
            .iter()
            .any(|symbol| self.remaining.starts_with(symbol.0))
        {
            let symbol = self.read_symbol();
            return TokenResult::Token(Token {
                range: symbol.1,
                ty: TokenTy::Symbol(symbol.0),
            });
        }

        if KEYWORDS
            .iter()
            .any(|keyword| self.remaining.starts_with(keyword.0))
        {
            let keyword = self.read_keyword();
            return TokenResult::Token(Token {
                range: keyword.1,
                ty: TokenTy::Keyword(keyword.0),
            });
        }

        let identifier_length = self
            .remaining
            .chars()
            .take_while(|&id_c| id_c.is_alphabetic() || id_c.is_numeric() || id_c == '_')
            .count();
        if identifier_length > 0 {
            let start_index = self.index;
            self.increment_char(identifier_length);
            self.increment_col(identifier_length);
            let range = start_index..start_index + identifier_length;
            return TokenResult::Token(Token {
                range,
                ty: TokenTy::Identifier,
            });
        }

        self.increment_char(1);
        self.increment_col(1);

        TokenResult::InvalidCharacter(c)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TextSection>;

    fn next(&mut self) -> Option<Self::Item> {
        let pre_line = self.line;
        let pre_col = self.column;
        let pre_index = self.index;
        match self.next_token() {
            TokenResult::Token(token) => Some(Ok(token)),
            TokenResult::Skip => return self.next(),
            TokenResult::InvalidCharacter(_) => {
                let mut copy = self.clone();

                let mut length = 0;
                // Yes this is terrible
                while let TokenResult::InvalidCharacter(_) = copy.next_token() {
                    length += 1;
                }

                self.increment_char(length);
                self.increment_col(length);

                let post_line = self.line;
                let post_col = self.column;
                let post_index = self.index;

                Some(Err(TextSection {
                    index: pre_index..post_index,
                    line: pre_line..post_line,
                    column: pre_col..post_col,
                }))
            }
            TokenResult::End => None,
        }
    }
}
