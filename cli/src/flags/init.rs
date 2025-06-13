use clap::Parser;

#[derive(Debug, Parser)]
pub struct InitFlags {
    /// Create a new project in this directory
    #[clap(short, long, value_name = "DIR", default_value = ".")]
    dir: String,

    /// Example template to use
    #[clap(short, long, value_name = "TEMPLATE", default_value = "basic")]
    template: String,

    /// Overwrite existing files if present
    #[clap(long)]
    force: bool,
}
