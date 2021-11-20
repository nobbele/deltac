use std::rc::Rc;

use crate::{
    lexer::token::{KeywordTy, SymbolTy, Token, TokenTy},
    PrimitiveTy,
};

#[derive(Debug, PartialEq)]
pub enum BinaryOperationTy {
    Addition,
    Greater,
    Assignment,
}

/*#[derive(Debug)]
pub enum UnaryOperationTy {
    Negate,
}*/

#[derive(Debug, PartialEq)]
pub enum Operation<'a> {
    /*Unary {
        target: Expresssion,
        ty: UnaryOperationTy,
    },*/
    Binary {
        left: Expression<'a>,
        ty: BinaryOperationTy,
        right: Expression<'a>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Literal<'a> {
    value: &'a str,
    ty: PrimitiveTy,
}

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Variable(&'a str),
    Literal(Literal<'a>),
    Operation(Rc<Operation<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum ControlFlow<'a> {
    If {
        condition: Expression<'a>,
        body: Vec<Statement<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Declaration {
        name: &'a str,
        expression: Expression<'a>,
    },
    Expression(Expression<'a>),
    ControlFlow(ControlFlow<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub arguments: Vec<()>,
    pub return_type: Option<&'a str>,
    // Block type?
    // Block == Expression?
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Module<'a> {
    pub functions: Vec<Function<'a>>,
}

#[derive(Debug)]
pub struct Parser<'a> {
    source: &'a str,
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a [Token]) -> Self {
        Parser {
            source,
            tokens,
            index: 0,
        }
    }
    pub fn parse(mut self) -> Module<'a> {
        let mut module = Module {
            functions: Vec::new(),
        };

        let Token { ty, .. } = self.read_token().unwrap();
        match ty {
            TokenTy::Keyword(KeywordTy::Function) => module.functions.push(self.parse_function()),
            _ => panic!("Unexpected Token!"),
        }
        module
    }

    fn read_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.index)?.clone();
        self.index += 1;
        Some(token)
    }

    fn peek_token(&mut self) -> Option<Token> {
        Some(self.tokens.get(self.index)?.clone())
    }

    fn parse_expression(&mut self) -> Expression<'a> {
        // TODO make this good
        let expr = match self.read_token().unwrap() {
            Token {
                ty: TokenTy::Literal(ty),
                range,
            } => Expression::Literal(Literal {
                value: &self.source[range],
                ty,
            }),
            Token {
                ty: TokenTy::Identifier,
                range,
            } => Expression::Variable(&self.source[range]),
            _ => panic!("Unexpected Token!"),
        };
        let next_expr = self.peek_token();
        match next_expr {
            Some(Token {
                ty: TokenTy::Symbol(op @ (SymbolTy::Add | SymbolTy::Greater)),
                ..
            }) => {
                self.read_token();
                let right = self.parse_expression();
                Expression::Operation(Rc::new(Operation::Binary {
                    left: expr,
                    ty: match op {
                        SymbolTy::Add => BinaryOperationTy::Addition,
                        SymbolTy::Greater => BinaryOperationTy::Greater,
                        _ => panic!("Unexpected Operation!"),
                    },
                    right,
                }))
            }
            None
            | Some(Token {
                ty: TokenTy::Symbol(SymbolTy::EndStmt | SymbolTy::OpBrace),
                ..
            }) => expr,
            t => panic!("Unexpected Token! {:?}", t),
        }
    }

    pub fn parse_body(&mut self) -> Vec<Statement<'a>> {
        let mut body = Vec::new();
        loop {
            let statement = match self.read_token().unwrap() {
                Token {
                    ty: TokenTy::Keyword(KeywordTy::Let),
                    ..
                } => {
                    let name = match self.read_token().unwrap() {
                        Token {
                            ty: TokenTy::Identifier,
                            range,
                        } => &self.source[range],
                        _ => panic!("Unexpected Token!"),
                    };
                    match self.read_token().unwrap() {
                        Token {
                            ty: TokenTy::Symbol(SymbolTy::Assign),
                            ..
                        } => {}
                        _ => panic!("Unexpected Token!"),
                    };
                    let expression = self.parse_expression();
                    match self.read_token().unwrap() {
                        Token {
                            ty: TokenTy::Symbol(SymbolTy::EndStmt),
                            ..
                        } => {}
                        _ => panic!("Unexpected Token!"),
                    };
                    Statement::Declaration { name, expression }
                }
                Token {
                    ty: TokenTy::Keyword(KeywordTy::If),
                    ..
                } => {
                    let condition = self.parse_expression();
                    match self.read_token().unwrap() {
                        Token {
                            ty: TokenTy::Symbol(SymbolTy::OpBrace),
                            ..
                        } => {}
                        _ => panic!("Unexpected Token!"),
                    };
                    let body = self.parse_body();
                    Statement::ControlFlow(ControlFlow::If { condition, body })
                }
                Token {
                    ty: TokenTy::Identifier,
                    range,
                } => {
                    match self.read_token().unwrap() {
                        Token {
                            ty: TokenTy::Symbol(SymbolTy::Assign),
                            ..
                        } => {}
                        _ => panic!("Unexpected Token!"),
                    };
                    let expr = self.parse_expression();
                    match self.read_token().unwrap() {
                        Token {
                            ty: TokenTy::Symbol(SymbolTy::EndStmt),
                            ..
                        } => {}
                        _ => panic!("Unexpected Token!"),
                    };
                    Statement::Expression(Expression::Operation(Rc::new(Operation::Binary {
                        left: Expression::Variable(&self.source[range]),
                        ty: BinaryOperationTy::Assignment,
                        right: expr,
                    })))
                }
                Token {
                    ty: TokenTy::Symbol(SymbolTy::ClBrace),
                    ..
                } => break,
                t => panic!("Unexpected Token! {:?}", t),
            };
            body.push(statement);
        }
        body
    }

    pub fn parse_function(&mut self) -> Function<'a> {
        let name = match self.read_token().unwrap() {
            Token {
                range,
                ty: TokenTy::Identifier,
            } => &self.source[range],
            _ => panic!("Unexpected Token!"),
        };
        let arguments = match (self.read_token().unwrap(), self.read_token().unwrap()) {
            (
                Token {
                    ty: TokenTy::Symbol(SymbolTy::OpParen),
                    ..
                },
                Token {
                    ty: TokenTy::Symbol(SymbolTy::ClParen),
                    ..
                },
            ) => Vec::new(),
            _ => panic!("Unexpected Token!"),
        };
        let return_type = match self.peek_token().unwrap() {
            Token {
                ty: TokenTy::Symbol(SymbolTy::ThinArrow),
                ..
            } => {
                self.read_token();
                let type_name = match self.read_token().unwrap() {
                    Token {
                        range,
                        ty: TokenTy::Identifier,
                    } => &self.source[range],
                    _ => panic!("Unexpected Token!"),
                };
                Some(type_name)
            }
            Token {
                ty: TokenTy::Symbol(SymbolTy::OpBrace),
                ..
            } => None,
            _ => panic!("Unexpected Token!"),
        };
        match self.read_token().unwrap() {
            Token {
                ty: TokenTy::Symbol(SymbolTy::OpBrace),
                ..
            } => {}
            _ => panic!("Unexpected Token!"),
        }
        let body = self.parse_body();
        Function {
            name,
            arguments,
            return_type,
            body,
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_expression_1() {
        let input = "12 + 2";
        let tokens = crate::lexer::tokenizer::Tokenizer::new(input)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut parser = Parser::new(input, &tokens);
        let expr = parser.parse_expression();
        assert_eq!(
            expr,
            Expression::Operation(Rc::new(Operation::Binary {
                left: Expression::Literal(Literal {
                    value: "12",
                    ty: PrimitiveTy::Number,
                }),
                ty: BinaryOperationTy::Addition,
                right: Expression::Literal(Literal {
                    value: "2",
                    ty: PrimitiveTy::Number,
                }),
            }))
        );
    }

    #[test]
    fn test_expression_2() {
        let input = "abc + 2";
        let tokens = crate::lexer::tokenizer::Tokenizer::new(input)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut parser = Parser::new(input, &tokens);
        let expr = parser.parse_expression();
        assert_eq!(
            expr,
            Expression::Operation(Rc::new(Operation::Binary {
                left: Expression::Variable("abc"),
                ty: BinaryOperationTy::Addition,
                right: Expression::Literal(Literal {
                    value: "2",
                    ty: PrimitiveTy::Number,
                }),
            }))
        );
    }
}
