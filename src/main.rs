use clap::Parser;
use csvutils::opts::options::App;
use csvutils::opts::options::SubCommand::*;

use csvutils::process::Processor;

fn main() -> anyhow::Result<()> {
    println!("A csvutils in Rust");

    let app = App::parse();
    println!("app: {:?}", app);
    handle_command(app)
}

fn handle_command(app: App) -> anyhow::Result<()> {
    match app.cmd {
        Dedup(opts) => {
            println!("dedup opts: {:?}", opts);
            opts.process()?;
        }
        Filter(opts) => {
            println!("filter opts: {:?}", opts);
            opts.process()?;
        }
        Merge(opts) => {
            println!("merge opts: {:?}", opts);
            opts.process()?;
        }
    }
    anyhow::Ok(())
}
