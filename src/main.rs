use clap::Parser;
use cmd::{SubCommands, CLI};
use inquire::Select;
use structs::GitHosts;

mod cmd;
mod select;
mod structs;

fn main() {
    let cli = CLI::parse();

    match cli.command {
        SubCommands::Init {} => init(),
        SubCommands::GenerateSchemas {} => generate_schemas(),
        SubCommands::Search {} => {}
    };
}

pub struct GitSchema {
    pub host_type: GitHosts,
    pub url: String,
}

fn init() {
    //TODO: Add more Items to the list
    let init_types = vec!["mod", "texturepacks"];
    let initans = Select::new("What type of Init", init_types).prompt();

    if initans.is_err() {
        return println!("Please Select an Option.");
    }

    let githostans = Select::new("Git Host?", GitHosts::VARIANTS.to_vec()).prompt();

    if githostans.is_err() {
        return println!("Please Select an Option.");
    }

    println!("Project Type: {:?}", initans.unwrap_or("mod"));
    println!("Githost: {:?}", githostans.unwrap_or(GitHosts::GitHub));
}

fn generate_schemas() {}
