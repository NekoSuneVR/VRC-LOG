use std::{collections::HashMap, time::Duration};

use anyhow::{bail, Result};
use reqwest::blocking::Client;

use crate::{
    discord::{DEVELOPER_ID, USER},
    provider::{Provider, Type},
};

const USER_AGENT: &str = concat!(
    "VRC-LOG/",
    env!("CARGO_PKG_VERSION"),
    " shaybox@shaybox.com"
);

pub struct NEKODB {
    client: Client,
    userid: String,
}

impl NEKODB {
    #[must_use]
    pub const fn new(client: Client, userid: String) -> Self {
        // TODO: Print NEKODB Statistics
        // Waiting on NEKODB Leaderboard

        Self { client, userid }
    }

    fn default() -> String {
        eprintln!("Error: Discord RPC Connection Failed\n");
        eprintln!("This may be due to one of the following reasons:");
        eprintln!("1. Discord is not running on your system.");
        eprintln!("2. VRC-LOG was restarted too quickly.\n");
        eprintln!("The User ID will default to the developer: NekoSuneVR");

        std::env::var("DISCORD").unwrap_or_else(|_| DEVELOPER_ID.to_owned())
    }
}

impl Default for NEKODB {
    fn default() -> Self {
        let client = Client::default();
        let userid = USER.clone().map_or_else(Self::default, |user| {
            let userid = user.id.unwrap_or_else(Self::default);
            if userid == "100463282099326976" {
                eprintln!("Vesktop & arRPC do not support fetching user info.");
                eprintln!("The User ID will default to the developer: NekoSuneVR");

                std::env::var("DISCORD").unwrap_or_else(|_| DEVELOPER_ID.to_owned())
            } else {
                if let Some(username) = user.username {
                    println!("[{}] Authenticated as {username}", Type::NEKODB);
                }

                userid
            }
        });

        Self::new(client, userid)
    }
}

impl Provider for NEKODB {
    fn check_avatar_id(&self, _avatar_id: &str) -> Result<bool> {
        bail!("Unsupported")
    }

    fn send_avatar_id(&self, avatar_id: &str) -> Result<bool> {
        let status = self
            .client
            .post("https://avtr.nekosunevr.co.uk/v1/vrchat/avatarapi/putavatarExternal")
            .header("User-Agent", USER_AGENT)
            .json(&HashMap::from([
                ("id", avatar_id),
                ("userid", &self.userid),
            ]))
            .send()?
            .status();

        if status == 429 {
            println!("[{}] 429 Rate Limit, Please Wait 1 Minute...", Type::NEKODB);
            std::thread::sleep(Duration::from_secs(60));
            self.send_avatar_id(avatar_id)
        } else {
            Ok(status == 201)
        }
    }
}
