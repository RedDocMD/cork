use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate pest;
#[macro_use]
extern crate pest_derive;
mod expression;
mod format;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut of = format::OutputFormat::default();
    loop {
        match rl.readline("cork> ") {
            Ok(line) => match expression::parse_line(&line) {
                Ok(command) => match command {
                    expression::Command::Expr(expr) => match expression::eval::eval_expr(&expr) {
                        Ok(val) => println!("{}", format::fmt(val, &of)),
                        Err(err) => eprintln!("Failed to evaluate \"{}\": {}", line, err),
                    },
                    expression::Command::Set(set) => {
                        if set[0] == "of" {
                            match set[1].as_str() {
                                "hex" => of.set_format_style(format::FormatStyle::Hex),
                                "dec" => of.set_format_style(format::FormatStyle::Decimal),
                                "oct" => of.set_format_style(format::FormatStyle::Octal),
                                "bin" => of.set_format_style(format::FormatStyle::Binary),
                                _ => eprintln!("Invalid {} value for key {}", set[1], set[0]),
                            }
                        } else {
                            eprintln!("{} is not a valid key", set[0]);
                        }
                    }
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
