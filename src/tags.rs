use super::api::ApiClient;
use structopt::StructOpt;
use fuzzy_filter::FuzzyFilter;
use twitch_api2::helix::tags::TwitchTag;

#[derive(StructOpt)]
#[structopt(about = "manipulate a streams tags")]
pub struct TagsOptions {
    #[structopt(short = "i", long, default_value = "en-us")]
    pub locale: String,

    #[structopt(subcommand)]
    pub subcommand: TagsSubcommand,
}

#[derive(StructOpt)]
pub enum TagsSubcommand {
    ListAll {
        #[structopt(flatten)]
        shared: SharedTagsOptions,
    },
    List {
        #[structopt(flatten)]
        shared: SharedTagsOptions,
        broadcaster: String,
    },
}

#[derive(StructOpt)]
pub struct SharedTagsOptions {
    #[structopt(short)]
    long: bool,
    #[structopt(short, long)]
    search_string: Option<String>,
}

pub async fn tags<'a>(client: ApiClient<'a>, locale: &str, command: TagsSubcommand) {
    match command {
        TagsSubcommand::ListAll { shared: SharedTagsOptions { long, search_string: needle } } => {
            let tags = client.get_all_tags().await.expect("valid tags");
            list(
                &tags,
                locale,
                needle.as_ref(),
                long,
            )
        }
        TagsSubcommand::List { shared: SharedTagsOptions { long, search_string: needle }, broadcaster } => {
            let tags = client.get_stream_tags(broadcaster).await.expect("valid tags");
            list(
                &tags,
                locale,
                needle.as_ref(),
                long,
            )
        }
    }
}

fn list<'a>(tags: &[TwitchTag], locale: &'a str, needle: Option<&'a String>, long: bool) {
    let filter = if let Some(needle) = needle {
        FuzzyFilter::new(needle)
    } else {
        FuzzyFilter::new("")
    };
    let max_len = tags
        .iter()
        .map(|tag| {
            tag.localization_names
                .get(locale)
                .expect("Specified locale is not available")
                .len()
        })
        .max()
        .unwrap_or(0);
    for tag in tags {
        let tag_name = tag.localization_names.get(locale).expect("valid locale");
        if filter.matches(tag_name) {
            if long {
                let tag_description = tag
                    .localization_descriptions
                    .get(locale)
                    .expect("valid locale");
                println!(
                    "'{}'{}{}",
                    tag_name,
                    " ".repeat(1 + max_len.saturating_sub(tag_name.len())),
                    tag_description
                );
            } else {
                print!("'{}' ", tag_name);
            }
        }
    }
}
