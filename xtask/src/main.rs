use clap::{Parser, Subcommand};
use std::{
  env,
  path::{Path, PathBuf},
};

#[cfg(feature = "lunarservermapping")]
mod lunar;

#[derive(Parser)]
struct ClI {
    command: Command,
}

enum Command {
    #[cfg(feature = "lunarservermapping")]
    LunarServerMappingConvert,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();
    env::set_current_dir(project_root())?;

    match command {
        #[cfg(feature = "lunarservermapping")]
        Command::LunarServerMappingConvert => lunar::servermapGenerate(); 
    }
}


fn project_root() -> PathBuf {
  Path::new(
    &env::var("CARGO_MANIFEST_DIR")
      .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
  )
  .ancestors()
  .nth(1)
  .unwrap()
  .to_path_buf()
}
