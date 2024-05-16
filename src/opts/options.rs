use clap::Parser;

use super::{dedup::DedupOptions, filter::FilterOptions, merge::MergeOptions};

// First set a cmd struct
#[derive(Debug, Parser)]
#[command(name = "csvutils")]
pub struct App {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "dedup")]
    Dedup(DedupOptions),
    #[command(name = "filter")]
    Filter(FilterOptions),
    #[command(name = "merge")]
    Merge(MergeOptions),
}
