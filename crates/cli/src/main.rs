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
use std::{fs::File, io::Write, str::FromStr, sync::LazyLock};

use chrono::Datelike;
use clap::Parser;
use cmd::{CLI, SubCommands};
//use generatedschemas::{ModLoaders, RegistryInfo, RegistryItem, Versioning};
//use inquire::{Confirm, MultiSelect, Select, Text, required};
use reqwest::{Client, ClientBuilder};
//use utils::{collect_socials, image, repo};

mod cmd;
mod prompts;
//mod schemas;
//mod utils;

static API_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("API_URL").unwrap_or_else(|_| "https://api.teaclient.net".to_string())
});

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    let current_year = chrono::Local::now().year();
    let client = ClientBuilder::new().build().unwrap();

    println!("    TeaClient  Copyright (C) {}  TeaClientMC", current_year);
    println!("    This program comes with ABSOLUTELY NO WARRANTY; for details type 'license'.");
    println!("    This is free software, and you are welcome to redistribute it");
    println!("    under certain conditions; type 'license' for details.");

    match cli.command {
        //SubCommands::Init {} => init(client).await,
        //SubCommands::Search {} => {}
        SubCommands::Licence {} => {
            print_license(current_year);
        }
    }
}

fn print_license(current_year: i32) {
    println!(
        r#"Provides Mods/Texturepacks/Profiles that are not available on modrinth/curseforge and servers to list from for TeaClient
Copyright (C) {}  TeaClient

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>."#,
        current_year
    );
}

// async fn init(client: Client) {
//     let init_types = vec!["mod", "server", "image", "texture_pack"];
//     let init_choice = Select::new("What type of Init?", init_types).prompt();
//
//     match init_choice {
//         Ok(choice) => match choice {
//             // "mod" => init_registry_item(client, "mods".to_string())
//             //     .await
//             //     .unwrap(),
//             "server" => init_server(client).await,
//             "image" => {
//                 let _ = image(false);
//             }
//             &_ => todo!(),
//         },
//         Err(_) => println!("Please select an option."),
//     }
// }
//
// pub async fn init_registry_item(
//     client: Client,
//     item_type: String,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let name = Text::new("What is the name of the item you want to add?")
//         .with_validator(required!())
//         .prompt()?;
//
//     let version = Text::new("What is the current version of the item you want to add?")
//         .with_validator(required!())
//         .prompt()?;
//
//     let desc = Text::new("What is a simple description of the item?")
//         .with_help_message("Checkout https://teaclient.net/wiki/registry#simple-description")
//         .prompt_skippable()?;
//
//     let minecraft_versions = vec!["1.7", "1.8.9", "1.12", "1.16", "1.20", "1.20.1", "1.21"]
//         .into_iter()
//         .map(String::from)
//         .collect::<Vec<String>>();
//
//     let mc_version = MultiSelect::new(
//         "What Minecraft version does your item support?",
//         minecraft_versions,
//     )
//     .prompt()?;
//
//     let modloader_variants: Vec<String> = client
//         .get(format!("{}/registry/mod-loaders", *API_URL))
//         .send()
//         .await?
//         .json()
//         .await?;
//
//     let selected_modloaders = MultiSelect::new(
//         "Which modloaders does your item support?",
//         modloader_variants,
//     )
//     .prompt()?;
//
//     let modloaders: Vec<ModLoaders> = selected_modloaders
//         .into_iter()
//         .map(|s| ModLoaders::from_str(&s).expect("Invalid modloader"))
//         .collect();
//
//     let is_repo = Confirm::new("Is this item in a Git repo?")
//         .with_default(true)
//         .prompt()?;
//
//     let downloading = repo(client, is_repo).await?;
//
//     let versioning = {
//         let version_regex_input =
//             Text::new("Enter the version regex (leave empty if none):").prompt()?;
//
//         let semantic_version = Confirm::new("Is it semantic versioning?")
//             .with_default(true)
//             .prompt()?;
//
//         let zero_version = Confirm::new("Does it allow zero versioning (e.g., 0.x.y)?")
//             .with_default(false)
//             .prompt()?;
//
//         let version_regex = if version_regex_input.trim().is_empty() {
//             None
//         } else {
//             Some(version_regex_input)
//         };
//
//         Versioning {
//             version_regex,
//             semantic_version,
//             zero_version,
//         }
//     };
//
//     let registryitem = RegistryItem {
//         info: RegistryInfo {
//             name,
//             version,
//             desc,
//             type_: item_type.clone(),
//             minecraft_version_support: mc_version,
//             modloaders,
//         },
//         downloading,
//         versioning,
//     };
//
//     let toml_string = toml::to_string(&registryitem)?;
//
//     std::fs::create_dir_all(&item_type)?;
//     let file_path = format!("{}/{}.toml", item_type, registryitem.info.name);
//     let mut file = File::create(&file_path)?;
//     file.write_all(toml_string.as_bytes())?;
//
//     println!("Successfully created {}", file_path);
//
//     Ok(())
// }
//
// async fn init_server(client: Client) {
//     let name = Text::new("What is the name of the server?")
//         .with_validator(required!())
//         .prompt()
//         .expect("Please specify the server name.");
//
//     let maintainer = Text::new("Who is the maintainer of the server?")
//         .with_validator(required!())
//         .prompt()
//         .expect("Please specify a maintainer.");
//
//     let desc = Text::new("Simple description of the server?")
//         .with_help_message("Checkout https://teaclient.net/wiki/registry#simple-description")
//         .prompt_skippable()
//         .unwrap();
//
//     let server_address = Text::new("Server address/IP of the server?")
//         .with_validator(required!())
//         .prompt()
//         .expect("Please specify the server address.");
//
//     let partner_variants: Vec<String> = client
//         .get(format!("{}/registry/partners", *API_URL))
//         .send()
//         .await
//         .unwrap()
//         .json()
//         .await
//         .unwrap();
//
//     let partnered_with = MultiSelect::new("Select Server Partners", partner_variants)
//         .prompt_skippable()
//         .unwrap();
//
//     if !Confirm::new("Open file dialogs for icon and banner?")
//         .with_default(false)
//         .prompt()
//         .unwrap_or(false)
//     {
//         eprintln!("Server setup cancelled by user.");
//         return;
//     }
//
//     let icon = image(true).expect("Failed to get server icon");
//     let banner = image(true).expect("Failed to get banner image");
//
//     let socials = collect_socials();
//
//     let server = generatedschemas::ServerSchema {
//         info: generatedschemas::ServerInfoSchema {
//             maintainer,
//             desc,
//             server_address,
//             partnered_with,
//         },
//         base64_png: generatedschemas::ServerBase64Png { icon, banner },
//         socials,
//     };
//
//     let toml_string = toml::to_string(&server).unwrap();
//     let mut file = File::create(format!("{}.toml", name)).unwrap();
//     file.write_all(toml_string.as_bytes()).unwrap();
//
//     println!("Generated server: {:?}", server);
// }
