use std::{path::PathBuf, str::FromStr};

use structopt::{clap::Shell, StructOpt};
use crate::tags::TagsOptions;

/// A sane Twitch commandline interface
#[derive(Debug, StructOpt)]
#[structopt(name = "twitchctl")]
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
            ShellType::Zsh  => Shell::Zsh,
            ShellType::PowerShell => Shell::PowerShell,
            ShellType::Elvish => Shell::Elvish,
        }
    }
}
