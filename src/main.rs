use clap::Parser;

use gb::git;

#[derive(Parser)]
struct Cli {
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Cli::parse();
    let branch_name = git::list_branches(git::CURRENT_REPO)?;
    for b in branch_name.iter() {
        println!("{}", b)
    }
    Ok(())
}
