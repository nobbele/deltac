use deltac::tokenizer::Tokenizer;

fn main() {
    let input = r#"
fn IAmAFunction() {
    1 + 1 + 1 + ??
}"#;
    let tokenizer = Tokenizer::new(input);
    for result in tokenizer {
        match result {
            //Ok(token) => println!("Token: `{:?}`", token),
            Ok(_) => {}
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
}
