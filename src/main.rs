mod cli;

use cli::Cli;
use exitfailure::ExitFailure;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();

    Ok(())
}
