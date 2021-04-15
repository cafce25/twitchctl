use super::api::ApiClient;
use fuzzy_filter::FuzzyFilter;
use structopt::StructOpt;
use twitch_api2::helix::tags::TwitchTag;

#[derive(Debug, StructOpt)]
/// manipulate a streams tags
pub struct TagsOptions {
    /// the locale to use for the tag names
    #[structopt(short, long, default_value = "en-us")]
    pub locale: String,

    #[structopt(subcommand)]
    pub subcommand: TagsSubcommand,
}

#[derive(Debug, StructOpt)]
pub enum TagsSubcommand {
    /// list all available tags
    ListAll {
        #[structopt(flatten)]
        shared: SharedTagsOptions,
    },
    /// list tags for a broadcaster
    List {
        /// the name of the broadcaster for which to list the tags
        ///
        /// if omitted the token user is used
        broadcaster: Option<String>,
        #[structopt(flatten)]
        shared: SharedTagsOptions,
    },
}

#[derive(Debug, StructOpt)]
pub struct SharedTagsOptions {
    /// print tags in long format
    #[structopt(short)]
    long: bool,
    /// string for fuzzy filtering of tags
    filter: Option<String>,
}

pub async fn tags<'a>(client: ApiClient<'a>, locale: &str, command: TagsSubcommand) {
    match command {
        TagsSubcommand::ListAll {
            shared: SharedTagsOptions { long, filter },
        } => {
            let tags = client.get_all_tags().await.expect("valid tags");
            list(&tags, locale, filter.as_ref(), long)
        }
        TagsSubcommand::List {
            shared: SharedTagsOptions { long, filter },
            broadcaster,
        } => {
            let tags = client
                .get_stream_tags(match broadcaster.as_ref() {
                    Some(b) => b,
                    None => client.get_user(),
                })
                .await
                .expect("valid tags");
            list(&tags, locale, filter.as_ref(), long)
        }
    }
}

fn list<'a>(tags: &[TwitchTag], locale: &'a str, filter_string: Option<&'a String>, long: bool) {
    let filter = if let Some(filter_string) = filter_string {
        FuzzyFilter::new(filter_string)
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
