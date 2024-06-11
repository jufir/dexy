use std::fs::File;

use dexy::Dex;

fn main() -> anyhow::Result<()> {
    let mut file = File::open("classes.dex")?;
    let dex = Dex::parse(&mut file)?;

    println!("{:?}", dex);

    Ok(())
}
