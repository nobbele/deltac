use deltac::{
    token::{KeywordTy, LiteralTy, NumberTy, SymbolTy, Token, TokenTy},
    tokenizer::{TextSection, Tokenizer},
};

#[test]
fn basic_1() {
    let v: Vec<Token> = Tokenizer::new("12 + 2").collect::<Result<_, _>>().unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            Token {
                range: 0..2,
                ty: TokenTy::Literal(LiteralTy::Number(NumberTy::Unspecified))
            },
            Token {
                range: 3..4,
                ty: TokenTy::Symbol(SymbolTy::Add)
            },
            Token {
                range: 5..6,
                ty: TokenTy::Literal(LiteralTy::Number(NumberTy::Unspecified))
            }
        ]
    )
}

#[test]
fn basic_2() {
    let v: Vec<Token> = Tokenizer::new("12 + 2\n+1")
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            Token {
                range: 0..2,
                ty: TokenTy::Literal(LiteralTy::Number(NumberTy::Unspecified))
            },
            Token {
                range: 3..4,
                ty: TokenTy::Symbol(SymbolTy::Add)
            },
            Token {
                range: 5..6,
                ty: TokenTy::Literal(LiteralTy::Number(NumberTy::Unspecified))
            },
            Token {
                range: 7..8,
                ty: TokenTy::Symbol(SymbolTy::Add)
            },
            Token {
                range: 8..9,
                ty: TokenTy::Literal(LiteralTy::Number(NumberTy::Unspecified))
            }
        ]
    )
}

#[test]
fn function_1() {
    let v: Vec<Token> = Tokenizer::new("fn IAmAFunction() {}")
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            Token {
                range: 0..2,
                ty: TokenTy::Keyword(KeywordTy::Function)
            },
            Token {
                range: 15..27,
                ty: TokenTy::Identifier
            },
            Token {
                range: 15..16,
                ty: TokenTy::Symbol(SymbolTy::OpParen)
            },
            Token {
                range: 16..17,
                ty: TokenTy::Symbol(SymbolTy::ClParen)
            },
            Token {
                range: 18..19,
                ty: TokenTy::Symbol(SymbolTy::OpBrace)
            },
            Token {
                range: 19..20,
                ty: TokenTy::Symbol(SymbolTy::ClBrace)
            }
        ]
    )
}

#[test]
fn basic_invalid_1() {
    let v: Vec<Result<_, _>> = Tokenizer::new("¤¤ 12 + 2 ¤¤").collect();
    assert_eq!(
        v.as_slice(),
        &[
            Err(TextSection {
                index: 0..2,
                line: 0..0,
                column: 0..2
            }),
            Ok(Token {
                range: 3..5,
                ty: TokenTy::Literal(LiteralTy::Number(NumberTy::Unspecified))
            }),
            Ok(Token {
                range: 6..7,
                ty: TokenTy::Symbol(SymbolTy::Add)
            }),
            Ok(Token {
                range: 8..9,
                ty: TokenTy::Literal(LiteralTy::Number(NumberTy::Unspecified))
            }),
            Err(TextSection {
                index: 10..12,
                line: 0..0,
                column: 10..12
            })
        ]
    )
}
