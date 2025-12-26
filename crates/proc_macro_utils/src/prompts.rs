use inquire::{Confirm, MultiSelect, Select, Text};

pub struct PromptCtx {
    pub mc_versions: Vec<String>,
    pub modloader_variants: Vec<String>,
}

pub trait Promptable: Sized {
    fn prompt_with(ctx: &PromptCtx) -> Result<Self, inquire::error::InquireError>;
}

impl Promptable for generated::RegistryItem {
    fn prompt_with(ctx: &PromptCtx) -> Result<Self, inquire::error::InquireError> {
        let name = Text::new("What is the name?").prompt()?;
        let version = Text::new("What is the current version?").prompt()?;
        let desc = Text::new("Simple description")
            .with_help_message("See https://teaclient.net/wiki/registry#simple-description")
            .prompt_skippable()?;

        let mc_version = MultiSelect::new(
            "What Minecraft version does your item support?",
            ctx.mc_versions.clone(),
        )
        .prompt()?;

        let selected_modloaders = MultiSelect::new(
            "Which modloaders does your item support?",
            ctx.modloader_variants.clone(),
        )
        .prompt()?;

        let semantic_version = Confirm::new("Is it semantic versioning?")
            .with_default(true)
            .prompt()?;
        let zero_version = Confirm::new("Allow zero versioning (0.x.y)?")
            .with_default(false)
            .prompt()?;
        let version_regex_input =
            Text::new("Enter the version regex (leave empty if none):").prompt()?;
        let version_regex = if version_regex_input.trim().is_empty() {
            None
        } else {
            Some(version_regex_input)
        };

        Ok(generated::RegistryItem {
            // fill fields according to the generated model layout
            // info, downloading/versioning, etc.
        })
    }
}
