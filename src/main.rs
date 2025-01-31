/**
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See thread GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 **/
use chrono::Datelike;
use clap::Parser;
use cmd::{SubCommands, CLI};
use generatedschemas::GitHosts;
use inquire::{required, ui::RenderConfig, MultiSelect, Select, Text};

mod cmd;
mod generatedschemas;

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    let current_year = chrono::Local::now().year();

    println!("    TeaClient  Copyright (C) {}  TeaClientMC", current_year);
    println!("    This program comes with ABSOLUTELY NO WARRANTY; for details type 'licence'.");
    println!("    This is free software, and you are welcome to redistribute it");
    println!("    under certain conditions; type 'licence' for details.");

    match cli.command {
        SubCommands::Init {} => init().await,
        SubCommands::Search {} => {}
        SubCommands::Licence {} => {
            println!(
                r#"    <one line to give the program's name and a brief idea of what it does.>
            Copyright (C) {}  TeaClient

            This program is free software: you can redistribute it and/or modify
            it under the terms of the GNU General Public License as published by
            the Free Software Foundation, either version 3 of the License, or
            (at your option) any later version.

            This program is distributed in the hope that it will be useful,
            but WITHOUT ANY WARRANTY; without even the implied warranty of
            MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
            GNU General Public License for more details.

            You should have received a copy of the GNU General Public License
            along with this program.  If not, see <http://www.gnu.org/licenses/>."#,
                current_year
            );
        }
    };
}

async fn init_registry_item(item_type: String) {
    let nameans = Text::new("What is the name of the item you want to add?")
        .with_validator(required!())
        .prompt();

    if nameans.is_err() {
        return println!("Please specify a name for this item");
    }

    let desc = Text::new("What is a simple description of the item that you want to add?")
        .with_default("Undefined")
        .with_help_message("Checkout https://teaclient.net/wiki/registry#simple-description");

    let desc = Text {
        message: "What is a simple description of the item you want to add?",
        default: Some("Undefined"),
        help_message: Some("Not Required"),
        formatter: Text::DEFAULT_FORMATTER,
        page_size: Text::DEFAULT_PAGE_SIZE,
        render_config: RenderConfig::default(),
        validators: Vec::new(),
        initial_value: None,
        autocompleter: None,
        placeholder: None,
    }
    .prompt()
    .unwrap();

    let minecraft_versions = vec!["1.7", "1.8.9", "1.12", "1.16", "1.20", "1.20.1", "1.21"];
    let mc_versionsans = MultiSelect::new(
        "What minecraft version does your item support?",
        minecraft_versions,
    )
    .prompt();
}
async fn init() {
    //TODO: Add more Items to the list
    let init_types = vec!["mod", "server"];
    let initans = Select::new("What type of Init", init_types).prompt();
    let client = reqwest::Client::new();
    match initans {
        Ok(choice) => match choice {
            "mod" => init_registry_item("mods".to_string()).await,
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
