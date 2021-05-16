use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate pest;
#[macro_use]
extern crate pest_derive;
mod expression;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline("cork> ") {
            Ok(line) => match expression::parse_line(&line) {
                Ok(command) => match command {
                    expression::Command::Expr(expr) => match expression::eval::eval_expr(&expr) {
                        Ok(val) => println!("{}", val),
                        Err(err) => eprintln!("Failed to evaluate \"{}\": {}", line, err),
                    },
                    expression::Command::Set(set) => eprintln!("Set directive: {}", set),
                    expression::Command::Empty => println!(""),
                },
                Err(err) => eprintln!("Failed to parse \"{}\": {}", line, err),
            },
            Err(ReadlineError::Eof) => {
                println!("Exiting ... ");
                break;
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl + C.\nPress Ctrl + D to exit");
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
