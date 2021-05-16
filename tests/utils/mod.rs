#![allow(unused)]

use discord_next::model::{ChannelId, ServerId};
use discord_next::Discord;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Mutex;

const ENV_BOT_TOKEN: &str = "DISCORD_BOT_TOKEN";
const ENV_SERVER_ID: &str = "DISCORD_SERVER_ID";
const ENV_TEXT_CHANNEL_ID: &str = "DISCORD_TEXT_CHANNEL_ID";

fn var<T, E>(var: &str) -> T
where
    T: FromStr<Err = E>,
    E: Debug,
{
    let txt = std::env::var(var).expect(&format!(
        "required environment variable `{}` was not set",
        var,
    ));

    txt.parse()
        .expect(&format!("could not parse environment variable `{}`", var,))
}

static START: spin::Once<Mutex<Environment>> = spin::Once::new();

pub struct Environment {
    pub discord: Discord,
    pub server: ServerId,
    pub text_channel: ChannelId,
}

impl Environment {
    fn try_connect() -> Result<Self, Box<dyn std::error::Error>> {
        let discord =
            Discord::from_bot_token(&var::<String, _>(ENV_BOT_TOKEN))?;

        Ok(Self {
            discord,
            server: var(ENV_SERVER_ID),
            text_channel: var(ENV_TEXT_CHANNEL_ID),
        })
    }

    pub fn connect() -> impl std::ops::DerefMut<Target = Self> {
        START
            .call_once(|| Mutex::new(Self::try_connect().unwrap()))
            .lock()
            .unwrap()
    }
}

pub fn random_str() -> String {
    let mut txt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(25)
        .map(char::from)
        .collect();

    txt.make_ascii_lowercase();

    txt
}
