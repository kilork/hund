mod cli;

use cli::*;
use exitfailure::ExitFailure;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();

    match cli.command {
        Command::New => hund::command::new(),
        _ => unimplemented!(),
    }

    Ok(())
}
