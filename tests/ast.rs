use std::rc::Rc;

use deltac::{
    ast::{
        BinaryOperationTy, ControlFlow, Expression, Function, Literal, Module, Operation, Parser,
        Statement,
    },
    lexer::tokenizer::Tokenizer,
    PrimitiveTy,
};

#[test]
fn test_expression_1() {
    let input = "12 + 2";
    let tokens = Tokenizer::new(input)
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
    let tokens = Tokenizer::new(input)
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

#[test]
fn test_complex_1() {
    let input = "
fn IAmAFunction() -> int {
    let a = 10;
    if a > 5 {
        b = 5;
    }
}";
    let tokens = Tokenizer::new(input)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let parser = Parser::new(input, &tokens);
    let expr = parser.parse();
    assert_eq!(
        expr,
        Module {
            functions: vec![Function {
                name: "IAmAFunction",
                arguments: vec![],
                return_type: Some("int"),
                body: vec![
                    Statement::Declaration {
                        name: "a",
                        expression: Expression::Literal(Literal {
                            value: "10",
                            ty: PrimitiveTy::Number
                        })
                    },
                    Statement::ControlFlow(ControlFlow::If {
                        condition: Expression::Operation(Rc::new(Operation::Binary {
                            left: Expression::Variable("a"),
                            ty: BinaryOperationTy::Greater,
                            right: Expression::Literal(Literal {
                                value: "5",
                                ty: PrimitiveTy::Number
                            })
                        })),
                        body: vec![Statement::Expression(Expression::Operation(Rc::new(
                            Operation::Binary {
                                left: Expression::Variable("b"),
                                ty: BinaryOperationTy::Assignment,
                                right: Expression::Literal(Literal {
                                    value: "5",
                                    ty: PrimitiveTy::Number
                                })
                            }
                        )))]
                    })
                ]
            }]
        }
    );
}
