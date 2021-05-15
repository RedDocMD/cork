use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    println!("Hello, world!");

    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline("cork> ") {
            Ok(line) => {
                println!("{}", line);
            }
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
