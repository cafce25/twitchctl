use crate::api::{get_broadcaster_id_or_die, ApiClient};
use crate::exit;
use fuzzy_filter::FuzzyFilter;
use structopt::StructOpt;
use twitch_api2::helix::points::{CreateCustomRewardBody, CustomReward, UpdateCustomRewardBody};
use twitch_api2::types::{UserId, UserName};

#[derive(Debug, StructOpt)]
/// manipulate a streams tags
pub struct RewardsOptions {
    #[structopt(subcommand)]
    pub subcommand: RewardsSubcommand,
}

#[derive(Debug, StructOpt)]
pub enum RewardsSubcommand {
    /// list rewards for a broadcaster
    List {
        /// the name of the broadcaster for which to list the rewards
        ///
        /// if omitted the token user is used
        #[structopt(flatten)]
        broadcaster: BroadcasterOption,
        /// print rewards in long format
        #[structopt(short)]
        long: bool,
        /// string for fuzzy filtering of tags
        filter: Option<String>,
    },
    /// creates a new reward for a broadcaster
    Add {
        #[structopt(flatten)]
        broadcaster: BroadcasterOption,
        #[structopt(flatten)]
        reward: RewardOption,
    },
    /// creates a new reward for a broadcaster
    Update {
        current_title: String,
        #[structopt(flatten)]
        broadcaster: BroadcasterOption,
        #[structopt(flatten)]
        reward: RewardOption,
    },
}

#[derive(Debug, StructOpt)]
pub struct RewardOption {
    /// the title of the reward
    #[structopt(short, long)]
    title: Option<String>,
    /// the cost of the reward
    #[structopt(short = "c", long)]
    cost: Option<usize>,
    /// the prompt for the viewer when redeeming the reward
    #[structopt(short = "d", long)]
    prompt: Option<String>,
    /// enable the reward, default for new rewards
    #[structopt(short, long, conflicts_with = "disabled")]
    enabled: bool,
    /// disable the reward
    #[structopt(short = "E", long, conflicts_with = "enabled")]
    disabled: bool,
    /// custom background color for the reward.
    ///
    /// Format: Hex with # prefix. Example: #00E5CB.
    #[structopt(short = "C", long)]
    color: Option<String>,
    /// enable user input
    #[structopt(short = "i", long)]
    user_input: bool,
    /// disable user input, default for new rewards
    #[structopt(short = "I", long)]
    no_user_input: bool,
    /// the maximum number per stream, default disabled for new rewards
    ///
    /// Set to 0 to disable.
    #[structopt(short, long)]
    max_per_stream: Option<usize>,
    /// the maximum number per user per stream, default disabled for new rewards
    ///
    /// Set to 0 to disable.
    #[structopt(short = "u", long)]
    max_per_user: Option<usize>,
    /// the cooldown in seconds, default disabled for new rewards
    ///
    /// Set to 0 to disable.
    #[structopt(short = "w", long)]
    cooldown: Option<usize>,
    /// redemptions are fulfilled immediately when redeemed
    #[structopt(short, long)]
    auto_fulfill: bool,
    /// redemptions are set to unfulfilled when redeemed, default for new rewards
    #[structopt(short = "A", long)]
    no_auto_fulfill: bool,
    /// is paused
    #[structopt(short, long)]
    paused: bool,
    /// is not paused, default for new rewards
    #[structopt(short = "P", long)]
    not_paused: bool,
}

impl From<RewardOption> for UpdateCustomRewardBody {
    fn from(reward: RewardOption) -> Self {
        let RewardOption {
            title,
            cost,
            prompt,
            enabled,
            disabled,
            color,
            user_input,
            no_user_input,
            max_per_stream,
            max_per_user,
            cooldown,
            auto_fulfill,
            no_auto_fulfill,
            paused,
            not_paused,
        } = reward;
        UpdateCustomRewardBody::builder()
            .title(title)
            .cost(cost)
            .prompt(prompt)
            .is_enabled(match (enabled, disabled) {
                (true, _) => Some(true),
                (_, true) => Some(false),
                _ => None,
            })
            .background_color(color)
            .is_user_input_required(match (user_input, no_user_input) {
                (true, _) => Some(true),
                (_, true) => Some(false),
                _ => None,
            })
            .is_max_per_stream_enabled(match max_per_stream {
                None => None,
                Some(0) => Some(false),
                _ => Some(true),
            })
            .max_per_stream(if max_per_stream.unwrap_or(0) != 0 {
                max_per_stream
            } else {
                None
            })
            .is_max_per_user_per_stream_enabled(match max_per_user {
                None => None,
                Some(0) => Some(false),
                _ => Some(true),
            })
            .max_per_user_per_stream(if max_per_user.unwrap_or(0) != 0 {
                max_per_user
            } else {
                None
            })
            .is_global_cooldown_enabled(match cooldown {
                None => None,
                Some(0) => Some(false),
                _ => Some(true),
            })
            .global_cooldown_seconds(if cooldown.unwrap_or(0) != 0 {
                cooldown
            } else {
                None
            })
            .should_redemptions_skip_request_queue(match (auto_fulfill, no_auto_fulfill) {
                (true, _) => Some(true),
                (_, true) => Some(false),
                _ => None,
            })
            .is_paused(match (paused, not_paused) {
                (true, _) => Some(true),
                (_, true) => Some(false),
                _ => None,
            })
            .build()
    }
}
impl From<RewardOption> for CreateCustomRewardBody {
    fn from(r: RewardOption) -> Self {
        match r {
            RewardOption {
                title: Some(title),
                cost: Some(cost),
                prompt,
                disabled,
                color,
                user_input,
                max_per_stream,
                max_per_user,
                cooldown,
                auto_fulfill,
                ..
            } => CreateCustomRewardBody::builder()
                .title(title)
                .cost(cost)
                .prompt(prompt)
                .is_enabled(!disabled)
                .background_color(color)
                .is_user_input_required(user_input)
                .is_max_per_stream_enabled(max_per_stream.unwrap_or(0) != 0)
                .max_per_stream(if max_per_stream.unwrap_or(0) != 0 {
                    max_per_stream
                } else {
                    None
                })
                .is_max_per_user_per_stream_enabled(max_per_user.unwrap_or(0) != 0)
                .max_per_user_per_stream(if max_per_user.unwrap_or(0) != 0 {
                    max_per_user
                } else {
                    None
                })
                .is_global_cooldown_enabled(cooldown.unwrap_or(0) != 0)
                .global_cooldown_seconds(if cooldown.unwrap_or(0) != 0 {
                    cooldown
                } else {
                    None
                })
                .should_redemptions_skip_request_queue(auto_fulfill)
                .build(),
            _ => exit!(1, "Title and cost are required to create a new reward."),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct BroadcasterOption {
    /// the name of the broadcaster
    #[structopt(short, long, conflicts_with = "broadcaster_id")]
    broadcaster: Option<UserName>,

    /// the id of the broadcaster
    #[structopt(long)]
    broadcaster_id: Option<UserId>,
}

pub async fn rewards(client: ApiClient<'_>, command: RewardsSubcommand) {
    match command {
        RewardsSubcommand::List {
            long,
            filter,
            broadcaster:
                BroadcasterOption {
                    broadcaster,
                    broadcaster_id,
                },
        } => {
            let id = get_broadcaster_id_or_die(&client, broadcaster, broadcaster_id).await;
            let rewards = client.get_rewards(&id);
            match rewards.await {
                Ok(rewards) => list(&rewards, filter, long),
                Err(e) => exit!(1, "An error occurred while fetching the rewards: {}", e),
            }
        }
        RewardsSubcommand::Add {
            broadcaster:
                BroadcasterOption {
                    broadcaster,
                    broadcaster_id,
                },
            reward,
        } => {
            let broadcaster_id =
                get_broadcaster_id_or_die(&client, broadcaster, broadcaster_id).await;

            // TODO Update Paused Status
            match client
                .create_custom_reward(&broadcaster_id, reward.into())
                .await
            {
                Ok(_) => {}
                Err(e) => exit!(1, "{}", e),
            }
        }
        RewardsSubcommand::Update {
            broadcaster:
                BroadcasterOption {
                    broadcaster,
                    broadcaster_id,
                },
            reward,
            current_title,
        } => {
            let broadcaster_id =
                get_broadcaster_id_or_die(&client, broadcaster, broadcaster_id).await;

            if let Some(CustomReward { id, title, .. }) = client
                .find_reward(&broadcaster_id, &current_title)
                .await
                .unwrap_or_else(|e| exit!(1, "{}", e))
            {
                match client
                    .update_custom_reward(&broadcaster_id, &id, reward.into())
                    .await
                {
                    Ok(_) => println!("Updated: `{}`", title),
                    Err(e) => exit!(1, "{}", e),
                }
            } else {
                exit!(
                    1,
                    "Did not find a unique reward matching `{}`",
                    current_title
                )
            }
        }
    }
}

fn list(rewards: &[CustomReward], filter: Option<String>, long: bool) {
    let filter = filter.as_ref().map(|f| f.to_lowercase());
    let filter = filter.as_ref().map(|f| FuzzyFilter::new(f));

    let max_len = rewards.iter().map(|r| r.title.len()).max().unwrap_or(0);

    for reward in rewards {
        let title = &reward.title;
        if match &filter {
            Some(filter) => filter.matches(&title.to_lowercase()),
            _ => true,
        } {
            if long {
                println!(
                    "'{}'{}{:?}",
                    title,
                    " ".repeat(1 + max_len.saturating_sub(title.len())),
                    reward
                );
            } else {
                print!("'{}' ", title);
            }
        }
    }
    if !long {
        println!()
    }
}
