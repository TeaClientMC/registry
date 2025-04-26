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
use std::{fs::File, io::Write, path::PathBuf, sync::LazyLock};

use chrono::Datelike;
use clap::Parser;
use cmd::{SubCommands, CLI};
use inquire::{required, Confirm, MultiSelect, Select, Text};
use reqwest::ClientBuilder;
use uuid::Uuid;

mod cmd;
mod generatedschemas;

static API_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("API_URL").unwrap_or_else(|_| "https://api.teaclient.net".to_string())
});

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    let current_year = chrono::Local::now().year();
    let client = ClientBuilder::new().build().unwrap();

    println!("    TeaClient  Copyright (C) {}  TeaClientMC", current_year);
    println!("    This program comes with ABSOLUTELY NO WARRANTY; for details type 'licence'.");
    println!("    This is free software, and you are welcome to redistribute it");
    println!("    under certain conditions; type 'licence' for details.");

    match cli.command {
        SubCommands::Init {} => init(client).await,
        SubCommands::Search {} => {}
        SubCommands::Licence {} => {
            println!(
                r#"    Provides Mods/Texturepacks/Profiles that are not avalible on modrinth/curseforge and servers to list from for TeaClient
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
        .with_help_message("Checkout https://teaclient.net/wiki/registry#simple-description")
        .prompt_skippable()
        .unwrap();

    let minecraft_versions = vec!["1.7", "1.8.9", "1.12", "1.16", "1.20", "1.20.1", "1.21"];
    let mc_versionsans = MultiSelect::new(
        "What minecraft version does your item support?",
        minecraft_versions,
    )
    .prompt();
}

fn prompt_socials(label: &str, help: &str) -> Option<String> {
    Text::new(label)
        .with_help_message(help)
        .prompt_skippable()
        .unwrap_or(None)
        .and_then(|input| {
            if input.trim().is_empty() {
                None
            } else {
                Some(input)
            }
        })
}

async fn init_server(client: reqwest::Client) {
    let name = Text::new("What is the name of the server?")
        .with_validator(required!())
        .prompt()
        .expect("Please specify the server name?");

    let maintainer = Text::new("Who is the maintainer of the server?")
        .with_validator(required!())
        .prompt()
        .expect("Please specify a maintainer");

    let desc = Text::new("What is a simple description of the server that you want to add?")
        .with_help_message("Checkout https://teaclient.net/wiki/registry#simple-description")
        .prompt_skippable()
        .unwrap();

    let server_address = Text::new("Server address/IP of the server?")
        .with_validator(required!())
        .prompt()
        .expect("Please specify the server address/IP");

    let partner_varients: Vec<String> = client
        .get(API_URL.to_owned() + "/registry/partners")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let partnered_with = MultiSelect::new("Select Server Partners", partner_varients)
        .prompt_skippable()
        .unwrap();

    let confirm = Confirm::new("This requires opening your native file dialog two times: one for the icon and one for the banner. Do you want to open it now?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    if !confirm {
        return eprintln!("Server setup cancelled by user");
    }

    let icon = image(true).expect("Failed to get server icon");
    let banner = image(true).expect("Failed to get banner image");

    let socials = loop {
        let homepage = prompt_socials("Homepage URL", "e.g. https://...");
        let store = prompt_socials("Store URL", "e.g. https://...");
        let discord = prompt_socials("Discord invite", "e.g. https://discord.gg/....");
        let twitter = prompt_socials("Twitter profile", "e.g. https://twitter.com/...");
        let youtube = prompt_socials("YouTube channel", "e.g. https://www.youtube.com/...");
        let tiktok = prompt_socials("TikTok account", "e.g. https://www.tiktok.com/...");
        let twitch = prompt_socials("Twitch channel", "e.g. https://www.twitch.tv/...");

        if Confirm::new("Looks good?")
            .with_default(true)
            .prompt()
            .unwrap_or(false)
        {
            break generatedschemas::Socials {
                homepage,
                store,
                discord,
                twitter,
                youtube,
                tiktok,
                twitch,
            };
        }
    };

    let server = generatedschemas::ServerSchema {
        info: generatedschemas::ServerInfoSchema {
            maintainer,
            desc,
            server_address,
            partnered_with,
        },
        base64_png: generatedschemas::ServerBase64Png { icon, banner },
        socials,
    };

    let toml_string = toml::to_string(&server).unwrap();

    let mut file = File::create(name + ".toml").unwrap();
    file.write_all(toml_string.as_bytes()).unwrap();

    // You can now use `server` object (e.g., send it somewhere, serialize, etc.)
    println!("Generated server: {:?}", server); // if it implements Debug
}

pub fn image(confirm_disable: bool) -> Result<String, Box<dyn std::error::Error>> {
    println!("\x1b[31mWarning avif on avif on transparent image doesn't fully work! If you want to help fix this then feel free to submit a pr.");
    println!("\nNote you need ffmpeg in your path or this will not work");
    println!("\x1b[37m");
    if confirm_disable != true {
        let confirm = Confirm::new(
            "This requires opening your native file dialog, do you want to open it now?",
        )
        .with_default(false)
        .prompt()?;

        if !confirm {
            return Err("User cancelled file dialog".into());
        }
    }

    let path: PathBuf = rfd::FileDialog::new()
        .pick_file()
        .ok_or("No file selected")?;

    let id = Uuid::new_v4().to_string();
    let output_dir = "images";

    for (ext, codec, extra_args) in [
        (
            "avif",
            "libaom-av1",
            Some(vec!["-pix_fmt", "rgba", "-crf", "30"]),
        ),
        ("webp", "libwebp", None),
    ] {
        let out = format!("{}/{}.{}", output_dir, id, ext);
        let mut cmd = std::process::Command::new("ffmpeg");
        cmd.args(["-i", path.to_str().unwrap(), "-c:v", codec]);

        if let Some(args) = extra_args {
            cmd.args(args);
        }

        cmd.arg(&out);
        cmd.status()?;
    }

    println!(
        "Generated images at: {0}/{1}.avif and {0}/{1}.webp",
        output_dir, id
    );

    Ok(id)
}

async fn init_image() {
    let _ = image(false);
}

async fn init(client: reqwest::Client) {
    //TODO: Add more Items to the list
    let init_types = vec!["mod", "server", "image", "texture_pack"];
    let initans = Select::new("What type of Init", init_types).prompt();
    match initans {
        Ok(choice) => match choice {
            "mod" => init_registry_item("mods".to_string()).await,
            // "texture_pack" => init_pack(),
            "server" => init_server(client).await,
            "image" => init_image().await,
            &_ => todo!(),
        },
        Err(_) => return println!("Please Select an Option."),
    };

    if initans.is_err() {
        return println!("Please Select an Option.");
    }
}
