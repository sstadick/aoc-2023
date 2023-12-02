use enum_dispatch::enum_dispatch;
use std::env;
use std::path::Path;
use std::process::Command;
use std::{error::Error, path::PathBuf};

type DynError = Box<dyn Error>;

use clap::Parser;

#[enum_dispatch]
trait CommandImpl {
    fn main(&self) -> Result<(), DynError>;
}

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    NewDay(NewDay),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}

// -------------- Tasks -----------

#[derive(Parser, Debug)]
struct NewDay {
    /// Create a new day
    #[clap(long, short)]
    name: String,
}

impl CommandImpl for NewDay {
    fn main(&self) -> Result<(), DynError> {
        let template = project_root().join("aoc").join("src").join("commands").join("day0.rs");
        let dest = project_root()
            .join("aoc")
            .join("src")
            .join("commands")
            .join(format!("{}.rs", self.name));

        let status = Command::new("cp")
            .current_dir(project_root())
            .args(&[template.to_str().unwrap(), dest.to_str().unwrap()])
            .status()?;
        if !status.success() {
            return Err("cargo install failed".into());
        }

        Ok(())
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}
