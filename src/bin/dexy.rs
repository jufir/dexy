use std::fs::File;
use clap::Parser;
use dexy::Dex;

#[derive(clap::Parser)]
struct Cli {
    /// DEX file to use
    file: std::path::PathBuf
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut file = File::open(args.file)?;
    let dex = Dex::parse(&mut file)?;

    println!("{:?}", dex);
    Ok(())
}
