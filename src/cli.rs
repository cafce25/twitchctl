use std::{path::PathBuf, str::FromStr};

use structopt::{clap::Shell, StructOpt};
use crate::tags::TagsOptions;

#[derive(StructOpt)]
#[structopt(name = "basic", about = "A sane Twitch commandline interface", version = "0.1.0")]
pub struct CliOptions {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(StructOpt)]
pub enum SubCommand {
    Tags { 
        #[structopt(flatten)]
        options: TagsOptions,
    },
    #[structopt(about = "searches in categories")]
    SearchCategories,
    #[structopt(about = "creates a file containing shell completions")]
    Completions {
        #[structopt(short, long, default_value = "completions")]
        target_dir: PathBuf,
        shell: ShellType,
    },
}

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
