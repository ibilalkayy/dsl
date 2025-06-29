use clap::Parser;

#[derive(Debug, Parser)]
pub struct ParseFlags {
    /// Give the input file
    #[clap(short, long, value_name = "FILE")]
    pub file: String,

    /// Output file (optional)
    #[clap(short, long, value_name = "OUTPUT_FILE")]
    pub output: Option<String>,

    /// Pretty-print the output
    #[clap(long)]
    pub pretty: bool,
}
