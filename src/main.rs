use clap::{App, Arg};
use config::{read_config, Config};
use error::CorkError;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::exit;

use crate::format::OutputFormat;

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod config;
mod error;
mod expression;
mod format;

fn main() {
    let app = App::new("cork")
        .version(version())
        .author("Deep Majumder <deep.majumder2019@gmail.com>")
        .about("Command-line calculator for hex-lovers")
        .arg(
            Arg::with_name("expr")
                .long("expr")
                .short("e")
                .takes_value(true)
                .value_name("EXPR")
                .help("evaluate <EXPR> and print it"),
        )
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .takes_value(true)
                .value_name("PATH")
                .help("load config file from <PATH>"),
        )
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .takes_value(true)
                .value_name("PATH")
                .help("load script file to run line by line"),
        );

    let matches = app.get_matches();

    let config = match read_config(matches.value_of("config")) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Failed to parse config: {}", err);
            exit(1);
        }
    };

    if let Some(expr_str) = matches.value_of("expr") {
        inline_evaluate(expr_str, &config);
    } else if let Some(file_path) = matches.value_of("file") {
        script_evaluate(file_path, &config);
    } else {
        interactive(&config);
    }
}

fn script_evaluate(file_path: &str, config: &Config) {
    let file = File::open(file_path);

    let file = match file {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    let lines = io::BufReader::new(file).lines();

    let mut ans = 0;
    let mut of = format::OutputFormat::default();
    of.set_format_style(*config.output_radix());

    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        };
        if line == "warranty" {
            warranty();
            continue;
        }
        let _ = match proccess_command(line, &mut ans, &mut of) {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };
    }
}

fn inline_evaluate(expr_str: &str, config: &Config) {
    match expression::parse_line(expr_str) {
        Ok(command) => match command {
            expression::Command::Expr(expr) => match expression::eval::eval_expr(&expr, 0) {
                Ok(ans) => println!(
                    "{}",
                    format::fmt(
                        ans,
                        &OutputFormat::from_format_style(*config.output_radix())
                    )
                ),
                Err(err) => {
                    eprintln!("Failed to evaluate \"{}\": {}", expr_str, err);
                    exit(1);
                }
            },
            expression::Command::Set(_) => {
                eprintln!("Set directive not allowed in inline-expression");
                exit(1);
            }
            expression::Command::Empty => {
                println!("Empty expression!")
            }
        },
        Err(err) => {
            eprintln!("Failed to parse \"{}\": {}", expr_str, err);
            exit(1);
        }
    }
}

fn interactive(config: &Config) {
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
                let _ = match proccess_command(line, &mut ans, &mut of) {
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("{}", e);
                    }
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

fn proccess_command(line: String, ans: &mut i64, of: &mut OutputFormat) -> Result<(), CorkError> {
    let command = expression::parse_line(&line)?;
    match command {
        expression::Command::Expr(expr) => {
            let val = expression::eval::eval_expr(&expr, *ans)?;
            *ans = val;
            println!("{}", format::fmt(val, of));
        }
        expression::Command::Set(set) => {
            if set[0] == "of" {
                match set[1].as_str() {
                    "hex" => of.set_format_style(format::FormatStyle::Hex),
                    "dec" => of.set_format_style(format::FormatStyle::Decimal),
                    "oct" => of.set_format_style(format::FormatStyle::Octal),
                    "bin" => of.set_format_style(format::FormatStyle::Binary),
                    _ => {
                        return Err(error::CorkError::InvalidValueForKey {
                            key: set[0].clone(),
                            value: set[1].clone(),
                        });
                    }
                }
            } else {
                return Err(error::CorkError::InvalidKey(set[0].clone()));
            }
        }
        expression::Command::Empty => println!(),
    };
    Ok(())
}

const LICENSE_HEADER: &str = "Copyright (C) 2021 Deep Majumder
This is free software; see the source code for copying conditions.
There is ABSOLUTELY NO WARRANTY; not even for MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.  For details, type 'warranty'.";

const WARRANTY: &str = "Copyright (C) 2021 Deep Majumder

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
