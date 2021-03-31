use super::api::ApiClient;
use clap::{App, Arg, ArgMatches};
use fuzzy_filter::FuzzyFilter;

pub fn get_tags_app() -> App<'static, 'static> {
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
}

pub async fn tags(
    client: ApiClient<'static>,
    matches: &ArgMatches<'static>,
) {
    match matches.subcommand() {
        ("listall", Some(sub_m)) => {
            listall(
                client,
                matches.value_of("locale").unwrap(),
                sub_m.value_of("needle"),
                sub_m.is_present("long"),
            )
            .await
        }
        _ => {},
    }
}

async fn listall<'a>(
    client: ApiClient<'static>,
    locale: &'a str,
    needle: Option<&str>,
    long: bool,
) {
    //let locale = "en-us";
    //let tags = client.get_all_tags().await.expect("valid tags");
    let filter = if let Some(needle) = needle {
        FuzzyFilter::new(needle)
    } else {
        FuzzyFilter::new("")
    };
    let max_len = tags
        //.iter()
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
