use clap::Parser;

#[derive(Debug, Parser)]
pub struct FilterOptions {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
    /// which field have what value will be keep
    #[arg(short, long)]
    pub field: String,
    /// which field have what value will not be keep
    #[arg(short, long)]
    pub not_field: String,
}
