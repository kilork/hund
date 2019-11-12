mod cli;

use cli::*;
use exitfailure::ExitFailure;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();

    match cli.command {
        Command::New { name } => hund::command::new(&name)?,
        Command::Publish => hund::command::publish()?,
        _ => unimplemented!(),
    }

    Ok(())
}
