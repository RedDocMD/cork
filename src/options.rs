use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Options {
    #[clap(
        short,
        long,
        value_name = "EXPR",
        help = "evaluate <EXPR> and print it",
        min_values = 1
    )]
    pub expr: Option<Vec<String>>,

    #[clap(
        short,
        long,
        help = "prints the output in all the bases (works only in expr evaluation mode)"
    )]
    pub all_bases: bool,

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
}
