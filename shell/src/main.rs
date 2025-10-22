use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

// use language_parsing::Parser;
use cupid_parser::lex;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let _ = rl.load_history("history.txt");

    let mut context = cupid_core::context::Context::default();

    loop {
        match rl.readline("q> ") {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if line == "\\q" {
                    break;
                }

                rl.add_history_entry(line).unwrap();

                let tokens = lex(line);

                println!("{:?}", tokens);

                let tokens = match tokens {
                    Ok(toks) => toks,
                    Err(e) => {
                        println!("Lexing error: {}", e);
                        continue;
                    }
                };

                if let Err(e) = context.load(tokens) {
                    println!("Context load error: {}", e);
                    continue;
                }

                loop {
                    match context.execute_next() {
                        Ok(result) => {
                            if context.is_complete() {
                                println!("Result: {:?}", result);
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Execution error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt").unwrap();
}
