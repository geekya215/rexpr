use rexpr::eval::Eval;
use rexpr::parser::Parser;
use rexpr::tokenizer::Tokenizer;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let eval = Eval::new();
    loop {
        let readline = rl.readline("rexpr> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut tokenizer = Tokenizer::new(&line);
                match tokenizer.tokenize() {
                    Ok(tokens) => match Parser::new(tokens).parse() {
                        Ok(node) => println!("{}", eval.eval(&node)),
                        Err(err) => println!("{:?}", err),
                    },
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}
