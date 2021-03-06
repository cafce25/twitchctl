use std::{path::PathBuf, str::FromStr};

use crate::rewards::RewardsOptions;
use crate::tags::TagsOptions;
use structopt::{
    clap::{AppSettings, Shell},
    StructOpt,
};

/// A sane Twitch commandline interface
#[derive(Debug, StructOpt)]
#[structopt(name = "twitchctl", global_settings = &[AppSettings::DeriveDisplayOrder])]
pub struct CliOptions {
    #[structopt(subcommand)]
    pub category: Category,
}

#[derive(Debug, StructOpt)]
pub enum Category {
    /// show all tags or tags for a specific broadcaster
    Tags {
        #[structopt(flatten)]
        options: TagsOptions,
    },
    /// searches in categories
    Search {
        /// max amount of results to show
        #[structopt(short, long, default_value = "20")]
        max_results: usize,
        /// the category in which to search
        category: String,
    },
    /// creates a file containing shell completions
    Completions {
        /// the directory to write the completion file to
        #[structopt(short, long, default_value = "completions")]
        target_dir: PathBuf,
        /// the shell for which the completions should be generated.
        /// (currently supported values: bash, fish, zsh, powershell, elvish)
        shell: ShellType,
    },
    /// applies a stream configuration from a config file
    ///
    /// Supported formats are TOML, INI, YAML, JSON.
    ///
    /// You can use environment variables prefixed with
    /// `TWITCHCTL_` to override settings in the config file.
    ///
    /// Environment variables prefixed with `TWITCHCTL_DEFAULT_`
    /// are taken as default value that will be overridden by both
    /// `TWITCHCTL_` variables and the config file.
    File {
        /// Environment variables will be ignored
        #[structopt(long)]
        noenv: bool,
        file: PathBuf,
    },
    /// applies a stream configuration from a preset
    ///
    /// Preset files are stored in the platform specific
    /// config folders and are matched fuzzily
    ///
    /// LINUX: `$XDG_CONFIG_HOME/twitchctl/presets/`
    ///
    /// WINDOWS: `{FOLDERID_RoamingAppData}\twitchctl\presets`
    ///
    /// MACOS: `$HOME/Library/Application Support`
    ///
    /// They follow the same syntax and restrictions as config files.
    ///
    /// They can also be overridden with environment variables.
    Preset {
        /// Environment variables will be ignored
        #[structopt(long)]
        noenv: bool,
        query: String,
    },
    /// creates or manages rewards
    Reward {
        #[structopt(flatten)]
        options: RewardsOptions,
    },
}

#[derive(Debug)]
pub enum ShellType {
    Bash,
    Fish,
    Zsh,
    PowerShell,
    Elvish,
}

impl FromStr for ShellType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(ShellType::Bash),
            "fish" => Ok(ShellType::Fish),
            "zsh" => Ok(ShellType::Zsh),
            "powershell" => Ok(ShellType::PowerShell),
            "elvish" => Ok(ShellType::Elvish),
            _ => Err("unsupported shell"),
        }
    }
}

impl Into<Shell> for &ShellType {
    fn into(self) -> Shell {
        match self {
            ShellType::Bash => Shell::Bash,
            ShellType::Fish => Shell::Fish,
            ShellType::Zsh => Shell::Zsh,
            ShellType::PowerShell => Shell::PowerShell,
            ShellType::Elvish => Shell::Elvish,
        }
    }
}
