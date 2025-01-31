use clap::Parser;
use cmd::{SubCommands, CLI};
use generatedschemas::GitHosts;
use inquire::Select;

mod cmd;
mod generatedschemas;

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    match cli.command {
        SubCommands::Init {} => init().await,
        SubCommands::Search {} => {}
    };
}

async fn init_registry_item(item_type: String, client: reqwest::Client) {}
async fn init() {
    //TODO: Add more Items to the list
    let init_types = vec!["mod", "server"];
    let initans = Select::new("What type of Init", init_types).prompt();
    let client = reqwest::Client::new();
    match initans {
        Ok(choice) => match choice {
            "mods" => init_registry_item("mods".to_string(), client),
            // "texture_pack" => init_pack(),
            // "server" => init_server(),
            &_ => todo!(),
        },
        Err(_) => return println!("Please Select an Option."),
    };

    if initans.is_err() {
        return println!("Please Select an Option.");
    }

    // let githostans = Select::new("Git Host?", GitHosts::VARIANTS.to_vec()).prompt();

    // if githostans.is_err() {
    //     return println!("Please Select an Option.");
    // }

    // println!("Project Type: {:?}", initans.unwrap_or("mod"));
    // println!("Githost: {:?}", githostans.unwrap_or(GitHosts::GitHub));
}
