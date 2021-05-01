mod cli;

use cli::{Cli, Command};
use structopt::StructOpt;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::from_args();

    match cli.command {
        Command::New { name } => hund::command::new(&name)?,
        Command::Publish => hund::command::publish()?,
        _ => unimplemented!(),
    }

    Ok(())
}
