use clap::{crate_version, Parser};
use config::{read_config, Config};
use error::CorkError;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::exit;
use strum::IntoEnumIterator;

use crate::{
    format::{FormatRadix, OutputFormat},
    options::Options,
};

#[macro_use]
extern crate lazy_static;

mod config;
mod error;
mod expression;
mod format;
mod options;

fn main() {
    let options = Options::parse();

    let mut config = match read_config(options.config.as_ref()) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Failed to parse config: {}", err);
            exit(1);
        }
    };
    config.override_from_options(&options);

    if let Some(expr_vec) = &options.expr {
        let expr_str = expr_vec.join(" ");
        inline_evaluate(&expr_str, &config, &options);
    } else if let Some(file_path) = &options.file {
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
    let mut of = OutputFormat::default()
        .with_format_radix(*config.output_radix())
        .with_punctuate_number(*config.punctuate_output());

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

fn inline_evaluate(expr_str: &str, config: &Config, options: &Options) {
    match expression::parse_line(expr_str) {
        Ok(command) => match command {
            expression::Command::Expr(expr) => match expression::eval::eval_expr(&expr, 0) {
                Ok(ans) => {
                    if options.all_bases {
                        for radix in FormatRadix::iter() {
                            println!(
                                "{:>21}: {}",
                                radix.to_string(),
                                OutputFormat::default()
                                    .with_format_radix(radix)
                                    .with_punctuate_number(*config.punctuate_output())
                                    .fmt(ans),
                            );
                        }
                    } else {
                        println!(
                            "{}",
                            OutputFormat::default()
                                .with_format_radix(*config.output_radix())
                                .with_punctuate_number(*config.punctuate_output())
                                .fmt(ans),
                        );
                    }
                }
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

    let mut of = OutputFormat::default()
        .with_format_radix(*config.output_radix())
        .with_punctuate_number(*config.punctuate_output());
    let mut ans = 0;
    loop {
        match rl.readline(config.prompt()) {
            Ok(line) => {
                rl.add_history_entry(&line);
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
            println!("{}", of.fmt(val));
        }
        expression::Command::Set(set) => {
            if set[0] == "of" {
                match set[1].as_str() {
                    "hex" => of.set_format_radix(FormatRadix::Hex),
                    "dec" => of.set_format_radix(FormatRadix::Decimal),
                    "oct" => of.set_format_radix(FormatRadix::Octal),
                    "bin" => of.set_format_radix(FormatRadix::Binary),
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
    println!("Cork, version {}", crate_version!());
    println!("{}\n", LICENSE_HEADER);
    println!("Welcome to cork - a calculator for hex-lovers!");
    println!("Press Ctrl + D to exit.");
}

fn warranty() {
    println!("Cork, version {}", crate_version!());
    println!("{}", WARRANTY);
}
