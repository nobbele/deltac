use std::collections::HashMap;
use std::process::{Command, exit};
use deltac::codegen::Generator;
use deltac::lexer::tokenizer::Tokenizer;
use deltac::parse::{BinaryOperationTy, ControlFlow, Expression, Operation, Statement};
use deltac::PrimitiveTy;

fn main() {
    let input = r#"
fn main() {
    let a = 10;
    if a > 5 {
        a = 5;
    }
    # return a;
}"#;
    let tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();
    for result in tokenizer {
        match result {
            Ok(token) => tokens.push(token),
            Err(section) => {
                if section.line.end != section.line.start {
                    println!(
                        "Invalid characters between {}:{} and {}:{} `{}`",
                        section.line.start,
                        section.column.start,
                        section.line.end,
                        section.column.end,
                        &input[section.index]
                    )
                } else {
                    let line_start = if section.line.start == 0 {
                        0
                    } else {
                        input
                            .match_indices('\n')
                            .nth(section.line.start - 1)
                            .unwrap()
                            .0
                            + 1
                    };
                    let line_end = match input.match_indices('\n').nth(section.line.start) {
                        Some((n, _)) => n,
                        None => input.len(),
                    };
                    println!("\u{001b}[31mError at line {}", section.line.start + 1);
                    println!("| \u{001b}[0m{}", &input[line_start..line_end]);
                    print!("\u{001b}[34m");
                    for _ in 0..section.column.start + 2 {
                        print!(" ");
                    }
                    if section.column.end - section.column.start > 1 {
                        print!("[");
                        for _ in 0..section.column.end - section.column.start - 2 {
                            print!("-");
                        }
                        print!("] these characters are invalid.");
                    } else {
                        print!("^ this character is invalid.")
                    }
                    println!("\u{001b}[0m");
                    /*println!(
                        "Invalid characters at line {} between columns {} and {} `{}`",
                        section.line.start,
                        section.column.start,
                        section.column.end,
                        &input[section.index]
                    )*/
                }
            }
        }
    }
    let parser = deltac::parse::Parser::new(input, &tokens);
    let module = parser.parse();

    let mut gen = Generator::new();
    gen.raw(".global main");
    gen.raw(".text");

    for func in module.functions {
        let func_name = if func.name == "main" {
            "main".to_owned()
        } else {
            format!("deltafunc_{}", func.name)
        };
        gen.label(&func_name);

        struct LocalVar {
            stack_index: u32,
            ty: PrimitiveTy,
        }

        // how to do dynamic locals??
        let mut locals: HashMap<&str, LocalVar> = HashMap::new();
        {
            let mut stack_counter = 8; // Offset by 8 for return value
            for stmt in &func.body {
                match stmt {
                    Statement::Declaration { name, expression } => {
                        let ty = expression.get_type(&|n| locals[n].ty);
                        locals.insert(name, LocalVar {
                            stack_index: stack_counter,
                            ty
                        });
                        stack_counter += match ty {
                            // Assume u32 for now
                            PrimitiveTy::Number => {
                                4
                            }
                            PrimitiveTy::Boolean => {
                                1
                            }
                            PrimitiveTy::Void => {
                                0
                            },
                        };
                    }
                    _ => {},
                }
            }

            stack_counter += 12; // Alignment
            stack_counter -= 8; // Refix original offset

            gen.raw(format!("sub ${}, %rsp", stack_counter));
        }

        let branch_counter = 0;

        for stmt in func.body {
            fn gen_expr<'a, 'b>(gen: &'a mut Generator, locals: &'a HashMap<&'b str, LocalVar>, expr: &'b Expression, out: Option<&'a LocalVar>) {
                match expr {
                    Expression::Variable(var) => {
                        if let Some(out) = out {
                            let var = &locals[var];
                            gen.raw(format!("movl -{}(%rsp), -{}(%rsp)", var.stack_index, out.stack_index));
                        }
                    }
                    Expression::Literal(lit) => {
                        if let Some(out) = out {
                            assert_eq!(out.ty, lit.ty);
                            match lit.ty {
                                PrimitiveTy::Number => {
                                    let v = lit.value.parse::<u32>().unwrap();
                                    gen.raw(format!("movl ${}, -{}(%rsp)", v, out.stack_index));
                                }
                                PrimitiveTy::Boolean => {
                                    let v: u32 = if lit.value.parse::<bool>().unwrap() {
                                        1
                                    } else {
                                        0
                                    };
                                    gen.raw(format!("movb ${}, -{}(%rsp)", v, out.stack_index));
                                }
                                PrimitiveTy::Void => {
                                    panic!()
                                }
                            }
                        }
                    }
                    Expression::Operation(op) => {
                        match &**op {
                            Operation::Binary { left, ty, right } => {
                                match ty {
                                    BinaryOperationTy::Assignment => {
                                        match left {
                                            Expression::Variable(var) => {
                                                gen_expr(gen, locals, right, Some(&locals[var]));
                                            }
                                            _ => todo!()
                                        }
                                    }
                                    _ => todo!()
                                }
                            }
                        }
                    }
                }
            }
            match stmt {
                Statement::Declaration { name, expression } => {
                    gen_expr(&mut gen, &locals, &expression, Some(&locals[name]));
                }
                Statement::Expression(expr) => gen_expr(&mut gen, &locals, &expr, None),
                Statement::ControlFlow(ctrl) => {
                    match ctrl {
                        ControlFlow::If { condition, body } => {
                            assert_eq!(condition.get_type(&|n| locals[n].ty), PrimitiveTy::Boolean);
                            match condition {
                                Expression::Variable(_var) => todo!(),
                                Expression::Literal(_lit) => todo!(),
                                Expression::Operation(op) => {
                                    match &*op {
                                        Operation::Binary { left, ty, right } => {
                                            let l_type = left.get_type(&|n| locals[n].ty);
                                            assert_eq!(l_type, right.get_type(&|n| locals[n].ty));
                                            assert_eq!(l_type, PrimitiveTy::Number);
                                            let gen_ref = |expr: &Expression| {
                                                match expr {
                                                    Expression::Variable(var) => format!("-{}(%rsp)", locals[var].stack_index),
                                                    Expression::Literal(lit) => {
                                                        assert_eq!(lit.ty, PrimitiveTy::Number);
                                                        format!("${}", lit.value)
                                                    },
                                                    Expression::Operation(_op) => todo!(),
                                                }
                                            };
                                            let l_ref = gen_ref(left);
                                            let r_ref = gen_ref(right);
                                            gen.raw(format!("cmpl {}, {}", r_ref, l_ref));
                                            let branch_name = format!("{}_{}", func_name, branch_counter);
                                            match ty {
                                                BinaryOperationTy::Greater => {
                                                    gen.raw(format!("jle {}", branch_name));
                                                }
                                                _ => panic!(),
                                            }
                                            for stmt in body {
                                                match stmt {
                                                    Statement::Expression(expr) => gen_expr(&mut gen, &locals, &expr, None),
                                                    _ => todo!()
                                                }
                                            }
                                            gen.label(branch_name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }

        gen.raw("mov $format, %rdi");
        gen.raw(format!("mov -{}(%rsp), %rsi", locals["a"].stack_index));
        gen.raw("xor %rax, %rax");
        gen.raw("call printf");

        if func.name == "main" {
            gen.exit(0);
        }
    }

    gen.label_with_value("format", ".asciz \"Value: %d\\n\"");

    let full_raw = gen.full_raw();
    std::fs::write("gen.s", full_raw).unwrap();

    let gcc_status = Command::new("gcc").arg("-no-pie").arg("gen.s").arg("-o").arg("a.out").status().unwrap();
    let gcc_code = gcc_status.code().unwrap();
    println!("GCC: {}", gcc_code);
    if gcc_code != 0 {
        exit(gcc_code);
    }
    println!("Running program!\n");

    let proc = Command::new("./a.out").spawn().unwrap();
    let output = proc.wait_with_output().unwrap();
    let proc_code = output.status.code().unwrap();
    println!("Exit Code: {}", proc_code);
    if proc_code != 0 {
        exit(proc_code);
    }
}
