pub mod git;
use crate::Cli;
use anyhow::{bail, Context, Result};
use clap::{ArgGroup, Args, CommandFactory, ErrorKind};

#[derive(Args)]
// make sure that only one argument must and will be used
#[clap(group(
	ArgGroup::new("vers")
		.required(true)
		.args(&["major", "minor", "patch"]),
))]
/// Semantic Versioning
pub(crate) struct SemVer {
    /// Path of the git repository
    #[clap(default_value_t = String::from("./"), long)]
    pub(crate) path: String,
    /// Auto increase Major (<major>.x.x)
    #[clap(long)]
    pub(crate) major: bool,
    /// Auto increase Minor (x.<minor>.x)
    #[clap(long)]
    pub(crate) minor: bool,
    /// Auto increase Patch (x.x.<patch>)
    #[clap(long)]
    pub(crate) patch: bool,
}

pub(crate) fn run(subcommand: SemVer) -> Result<()> {
    let res = git::get_latest_tag(subcommand.path.clone())?;
    log::info!("{}", res);
    let version = res.split(".").collect::<Vec<_>>();
    let (mut major, mut minor, mut patch) = (
        version[0].parse::<i32>().context("unable to convert")?,
        version[1].parse::<i32>().context("unable to convert")?,
        version[2].parse::<i32>().context("unable to convert")?,
    );
    log::info!("{} {} {}", major, minor, patch);

    match (subcommand.major, subcommand.minor, subcommand.patch) {
        (true, _, _) => major += 1,
        (_, true, _) => minor += 1,
        (_, _, true) => patch += 1,
        // it is safe to just use unreachable!() because u used the AppGroup on the SemVer struct
        // but for learning purposes the error handling will be not commented
        _ => {
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Can only modify one version field",
            )
            .exit();
        }
    }
    log::info!("{} {} {}", major, minor, patch);
    log::warn!("{} {} {}", major, minor, patch);
    log::error!("{} {} {}", major, minor, patch);
    let res = git::update_version(
        subcommand.path.clone(),
        format!("{}.{}.{}", major, minor, patch),
    )?;
    log::info!("{}", res);
    Ok(())
}
