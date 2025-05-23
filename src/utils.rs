use crate::{
    generatedschemas::{RegistryDownload, RegistryItemGit, RegistryItemNormal, Socials},
    API_URL,
};
use inquire::{Confirm, Select, Text};
use std::{path::PathBuf, process::Command};
use uuid::Uuid;

pub fn collect_socials() -> Socials {
    loop {
        let homepage = prompt_optional_text("Homepage URL", "e.g. https://...");
        let store = prompt_optional_text("Store URL", "e.g. https://...");
        let discord = prompt_optional_text("Discord Invite", "e.g. https://discord.gg/...");
        let twitter = prompt_optional_text("Twitter Profile", "e.g. https://twitter.com/...");
        let youtube = prompt_optional_text("YouTube Channel", "e.g. https://www.youtube.com/...");
        let tiktok = prompt_optional_text("TikTok Profile", "e.g. https://www.tiktok.com/...");
        let twitch = prompt_optional_text("Twitch Channel", "e.g. https://www.twitch.tv/...");

        if Confirm::new("Looks good?")
            .with_default(true)
            .prompt()
            .unwrap_or(false)
        {
            return Socials {
                homepage,
                store,
                discord,
                twitter,
                youtube,
                tiktok,
                twitch,
            };
        }
    }
}

fn prompt_optional_text(label: &str, help: &str) -> Option<String> {
    Text::new(label)
        .with_help_message(help)
        .prompt_skippable()
        .unwrap_or(None)
        .filter(|input| !input.trim().is_empty())
}

pub fn image(confirm_disable: bool) -> Result<String, Box<dyn std::error::Error>> {
    println!("Note: Requires `ffmpeg` and `ffprobe` in PATH.");

    if !confirm_disable
        && !Confirm::new("Open file dialog?")
            .with_default(false)
            .prompt()?
    {
        return Err("User cancelled file dialog.".into());
    }

    let path: PathBuf = rfd::FileDialog::new()
        .pick_file()
        .ok_or("No file selected.")?;

    let id = Uuid::new_v4().to_string();
    let output_dir = "images";

    // Detect alpha using ffprobe
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=pix_fmt",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path.to_str().unwrap(),
        ])
        .output()?;

    let pix_fmt = String::from_utf8_lossy(&output.stdout);
    let has_alpha = pix_fmt.contains("a"); // e.g., "yuva420p"

    let mut force_alpha = false;
    if !has_alpha {
        force_alpha = Confirm::new("FFmpeg could not detect alpha. Is this image transparent?")
            .with_default(false)
            .prompt()?;
    }

    for (ext, codec, extra_args) in [
        (
            "avif",
            "libaom-av1",
            if has_alpha || force_alpha {
                Some(vec![
                    "-map",
                    "0",
                    "-map",
                    "0",
                    "-filter:v:0",
                    "format=yuv444p10le",
                    "-filter:v:1",
                    "alphaextract,format=gray10le",
                ])
            } else {
                Some(vec!["-vf", "format=yuv444p10le"])
            },
        ),
        ("webp", "libwebp", None),
    ] {
        let output = format!("{}/{}.{}", output_dir, id, ext);
        let mut cmd = Command::new("ffmpeg");

        cmd.args(["-i", path.to_str().unwrap(), "-c:v", codec]);

        if let Some(args) = extra_args {
            cmd.args(args);
        }

        cmd.arg(&output);
        cmd.status()?;
    }

    println!(
        "Generated images at: {}/{}.avif and {}/{}.webp",
        output_dir, id, output_dir, id
    );
    Ok(id)
}

pub async fn repo(
    client: reqwest::Client,
    is_repo: bool,
) -> Result<RegistryDownload, Box<dyn std::error::Error>> {
    if is_repo {
        let host_variants: Vec<String> = client
            .get(format!("{}/registry/githost", *API_URL))
            .send()
            .await?
            .json()
            .await?;

        let selected_host = Select::new("Select Git Host:", host_variants).prompt()?;

        let host_type = selected_host.parse()?;
        let url = Text::new("Enter the Git repository URL:").prompt()?;

        Ok(RegistryDownload::Git(RegistryItemGit { host_type, url }))
    } else {
        let download_url = Text::new("Enter the download URL:").prompt()?;
        let version_url = Text::new("Enter the version URL:").prompt()?;
        let licence_input = Text::new("Enter the license (leave empty if none):").prompt()?;

        let licence = if licence_input.trim().is_empty() {
            None
        } else {
            Some(licence_input)
        };

        Ok(RegistryDownload::Normal(RegistryItemNormal {
            download_url,
            version_url,
            licence,
        }))
    }
}
