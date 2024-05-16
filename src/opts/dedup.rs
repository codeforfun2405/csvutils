use clap::Parser;

#[derive(Debug, Parser)]
pub struct DedupOptions {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
    #[arg(short, long)]
    /// which field is the field that can not be duplicate
    pub field: String,
}
