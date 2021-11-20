use deltac::{
    lexer::{
        token::{KeywordTy, SymbolTy, Token, TokenTy},
        tokenizer::Tokenizer,
    },
    PrimitiveTy, TextSection,
};

#[test]
fn basic_1() {
    let v: Vec<Token> = Tokenizer::new("12 + 2").collect::<Result<_, _>>().unwrap();
    assert_eq!(
        v.as_slice(),
        &[
            Token {
                range: 0..2,
                ty: TokenTy::Literal(PrimitiveTy::Number)
            },
            Token {
                range: 3..4,
                ty: TokenTy::Symbol(SymbolTy::Add)
            },
            Token {
                range: 5..6,
                ty: TokenTy::Literal(PrimitiveTy::Number)
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
                ty: TokenTy::Literal(PrimitiveTy::Number)
            },
            Token {
                range: 3..4,
                ty: TokenTy::Symbol(SymbolTy::Add)
            },
            Token {
                range: 5..6,
                ty: TokenTy::Literal(PrimitiveTy::Number)
            },
            Token {
                range: 7..8,
                ty: TokenTy::Symbol(SymbolTy::Add)
            },
            Token {
                range: 8..9,
                ty: TokenTy::Literal(PrimitiveTy::Number)
            }
        ]
    )
}

#[test]
fn complex_1() {
    let v: Vec<Token> = Tokenizer::new("fn FooBar() {\n let a = 10; \nif a > 5 {} \n}")
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
                range: 3..9,
                ty: TokenTy::Identifier
            },
            Token {
                range: 9..10,
                ty: TokenTy::Symbol(SymbolTy::OpParen)
            },
            Token {
                range: 10..11,
                ty: TokenTy::Symbol(SymbolTy::ClParen)
            },
            Token {
                range: 12..13,
                ty: TokenTy::Symbol(SymbolTy::OpBrace)
            },
            Token {
                range: 15..18,
                ty: TokenTy::Keyword(KeywordTy::Let)
            },
            Token {
                range: 19..20,
                ty: TokenTy::Identifier
            },
            Token {
                range: 21..22,
                ty: TokenTy::Symbol(SymbolTy::Assign)
            },
            Token {
                range: 23..25,
                ty: TokenTy::Literal(PrimitiveTy::Number)
            },
            Token {
                range: 25..26,
                ty: TokenTy::Symbol(SymbolTy::EndStmt)
            },
            Token {
                range: 28..30,
                ty: TokenTy::Keyword(KeywordTy::If)
            },
            Token {
                range: 31..32,
                ty: TokenTy::Identifier
            },
            Token {
                range: 33..34,
                ty: TokenTy::Symbol(SymbolTy::Greater)
            },
            Token {
                range: 35..36,
                ty: TokenTy::Literal(PrimitiveTy::Number)
            },
            Token {
                range: 37..38,
                ty: TokenTy::Symbol(SymbolTy::OpBrace)
            },
            Token {
                range: 38..39,
                ty: TokenTy::Symbol(SymbolTy::ClBrace)
            },
            Token {
                range: 41..42,
                ty: TokenTy::Symbol(SymbolTy::ClBrace)
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
                range: 3..15,
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
fn function_2() {
    let v: Vec<Token> = Tokenizer::new("fn IAmAFunction() -> int { return 5; }")
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
                range: 3..15,
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
                range: 18..20,
                ty: TokenTy::Symbol(SymbolTy::ThinArrow)
            },
            Token {
                range: 21..24,
                ty: TokenTy::Identifier
            },
            Token {
                range: 25..26,
                ty: TokenTy::Symbol(SymbolTy::OpBrace)
            },
            Token {
                range: 27..33,
                ty: TokenTy::Keyword(KeywordTy::Return)
            },
            Token {
                range: 34..35,
                ty: TokenTy::Literal(PrimitiveTy::Number)
            },
            Token {
                range: 35..36,
                ty: TokenTy::Symbol(SymbolTy::EndStmt)
            },
            Token {
                range: 37..38,
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
                ty: TokenTy::Literal(PrimitiveTy::Number)
            }),
            Ok(Token {
                range: 6..7,
                ty: TokenTy::Symbol(SymbolTy::Add)
            }),
            Ok(Token {
                range: 8..9,
                ty: TokenTy::Literal(PrimitiveTy::Number)
            }),
            Err(TextSection {
                index: 10..12,
                line: 0..0,
                column: 10..12
            })
        ]
    )
}
