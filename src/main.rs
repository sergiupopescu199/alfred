use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod semver;
use semver::SemVer;

#[derive(Parser)]
#[clap(name = "alfred")]
#[clap(author = "Sergiu Popescu <sergiupopescu2@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Does awesome things", long_about = None)]
#[clap(propagate_version = true)]

struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Semantic versioning trough git tags
    Semver(SemVer),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Semver(subcommand) => Ok(semver::run(subcommand)?),
    }
}
