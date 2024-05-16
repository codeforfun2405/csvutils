use clap::Parser;

#[derive(Debug, Parser)]
pub struct MergeOptions {
    #[arg(short, long, value_delimiter=' ', num_args = 1..)]
    pub inputs: Vec<String>,
    #[arg(short, long)]
    pub output: String,
}
