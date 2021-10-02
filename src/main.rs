use config::read_config;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::PathBuf;
use std::process::exit;

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod config;
mod expression;
mod format;

fn main() {
    let config = match read_config() {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Failed to parse config: {}", err);
            exit(1);
        }
    };

    if *config.header() {
        welcome();
    }

    let mut rl = Editor::<()>::new();
    let history_file_name = PathBuf::from(".cork_history");
    let home_dir = home::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut history_path = home_dir;
    history_path.push(history_file_name);

    if rl.load_history(&history_path).is_err() {
        println!("No existing history!\n");
    }

    let mut of = format::OutputFormat::default();
    of.set_format_style(*config.output_radix());
    let mut ans = 0;
    loop {
        match rl.readline(config.prompt()) {
            Ok(line) => {
                if line == "warranty" {
                    warranty();
                    continue;
                }
                rl.add_history_entry(line.as_str());
                match expression::parse_line(&line) {
                    Ok(command) => match command {
                        expression::Command::Expr(expr) => {
                            match expression::eval::eval_expr(&expr, ans) {
                                Ok(val) => {
                                    ans = val;
                                    println!("{}", format::fmt(val, &of));
                                }
                                Err(err) => eprintln!("Failed to evaluate \"{}\": {}", line, err),
                            }
                        }
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
                        expression::Command::Empty => println!(),
                    },
                    Err(err) => eprintln!("Failed to parse \"{}\": {}", line, err),
                };
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

    rl.save_history(&history_path).unwrap();
}

const LICENSE_HEADER: &'static str = "Copyright (C) 2021 Deep Majumder
This is free software; see the source code for copying conditions.
There is ABSOLUTELY NO WARRANTY; not even for MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.  For details, type 'warranty'.";

const WARRANTY: &'static str = "Copyright (C) 2021 Deep Majumder

Cork is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by
the Free Software Foundation, version 2 of the License.

Cork is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Cork; see the file LICENSE.  If not, see
<https://www.gnu.org/licenses/>.";

fn welcome() {
    println!("Cork, version {}", version());
    println!("{}\n", LICENSE_HEADER);
    println!("Welcome to cork - a calculator for hex-lovers!");
    println!("Press Ctrl + D to exit.");
}

fn version() -> &'static str {
    "0.2.0"
}

fn warranty() {
    println!("Cork, version {}", version());
    println!("{}", WARRANTY);
}
