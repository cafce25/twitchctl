use super::api::ApiClient;
use clap::{App, Arg, ArgMatches};
use fuzzy_filter::FuzzyFilter;
use twitch_api2::helix::tags::TwitchTag;

pub fn get_tags_app<'a>() -> App<'a, 'a> {
    App::new("tags")
        .about("manipulate a streams tags")
        .arg(
            Arg::with_name("locale")
                .short("i")
                .long("locale")
                .default_value("en-us"),
        )
        .subcommand(
            App::new("listall")
                .arg(
                    Arg::with_name("long")
                        .short("l")
                        .help("use a long listing format"),
                )
                .arg(
                    Arg::with_name("needle")
                        .index(1)
                        .help("filter tags with a fuzzy search"),
                ),
        )
        .subcommand(
            App::new("list")
                .arg(
                    Arg::with_name("long")
                        .short("l")
                        .help("use a long listing format"),
                )
                .arg(
                    Arg::with_name("broadcaster")
                        .index(1)
                        .required(true)
                        .help("the broadcaster you want to fetch tags for"),
                        //TODO default to the one running this command
                )
                .arg(
                    Arg::with_name("needle")
                        .index(2)
                        .help("filter tags with a fuzzy search"),
                ),
        )
}

pub async fn tags<'a>(client: ApiClient<'a>, matches: &ArgMatches<'a>) {
    match matches.subcommand() {
        ("listall", Some(sub_m)) => {
            let tags = client.get_all_tags().await.expect("valid tags");
            list(
                &tags,
                matches.value_of("locale").unwrap(),
                sub_m.value_of("needle"),
                sub_m.is_present("long"),
            )
        }
        ("list", Some(sub_m)) => {
            let tags = client.get_stream_tags(sub_m.value_of("broadcaster").unwrap().to_string()).await.expect("valid tags");
            list(
                &tags,
                matches.value_of("locale").unwrap(),
                sub_m.value_of("needle"),
                sub_m.is_present("long"),
            )
        }
        _ => {}
    }
}
fn list<'a>(tags: &[TwitchTag], locale: &'a str, needle: Option<&str>, long: bool) {
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
