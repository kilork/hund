use clap_verbosity_flag::Verbosity;
use structopt::StructOpt;

#[derive(StructOpt)]
/// The Hund
///
/// 🐶
pub struct Cli {
    #[structopt(flatten)]
    pub command: Command,
    #[structopt(flatten)]
    pub verbosity: Verbosity,
}

#[derive(StructOpt)]
pub enum Command {
    /// Create a new hund project
    New {
        /// Project's name
        name: String,
    },
    /// Create a new hund project in an existing directory
    Init,
    /// Install dependencies
    Install,
    /// Publish
    Publish,
}
