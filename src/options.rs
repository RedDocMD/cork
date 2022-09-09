use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(ArgGroup::new("base").args(&["all", "hex", "oct", "dec", "bin"])))]
pub struct Options {
    #[clap(
        short,
        long,
        value_name = "EXPR",
        help = "evaluate <EXPR> and print it",
        min_values = 1
    )]
    pub expr: Option<Vec<String>>,

    #[clap(short, long, help = "punctuate the output number")]
    pub punctuate_output: bool,

    #[clap(
        short,
        long,
        value_name = "PATH",
        help = "load config file from <PATH>"
    )]
    pub config: Option<String>,

    #[clap(
        short,
        long,
        value_name = "PATH",
        help = "load script file from <PATH> to run line by line"
    )]
    pub file: Option<String>,

    #[clap(short, long, help = "print in all bases (only in expr eval mode)")]
    pub all: bool,

    #[clap(short = 'x', long, help = "print in hex (only in expr eval mode)")]
    pub hex: bool,

    #[clap(short, long, help = "print in oct (only in expr eval mode)")]
    pub oct: bool,

    #[clap(short, long, help = "print in dec (only in expr eval mode)")]
    pub dec: bool,

    #[clap(short, long, help = "print in bin (only in expr eval mode)")]
    pub bin: bool,
}
