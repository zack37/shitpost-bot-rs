use super::*;

use emojis::format_emoji;
use rand::{thread_rng, Rng};
use regex::Regex;
use serde::Deserialize;
use serenity::{
    futures::try_join,
    model::{
        channel::{Message, ReactionType},
        guild::Emoji,
        id::UserId,
    },
    utils::MessageBuilder,
};
use std::fmt::Debug;

lazy_static! {
    static ref MAGA_RE: Regex = Regex::new("(?i)ma[dk]e .* great again").unwrap();
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    url: String,
}

pub struct Reply {
    context: Context,
    msg: Message,
}

type DiscordResult = Result<(), serenity::Error>;

impl Reply {
    pub fn new(context: Context, msg: Message) -> Reply {
        Reply { context, msg }
    }

    pub async fn reply(&self) -> Result<(), Box<dyn std::error::Error>> {
        try_join!(
            // reaction replies
            self.wendy_parrot(),
            self.bepsi(),
            self.maga(),
            self.gkappa(),
            self.gzack(),
            self.zack(),
            self.trumpgasm(),
            self.maga_react(),
            self.rizo_pls(),
            self.sick(),
            self.friday(),
            self.dog(),
            // simple replies
            self.fuck_you(),
            self.steam(),
            self.mention_bacon(),
            self.mention_zack(),
            self.nsa(),
            // unique replies
            self.random_rizo_reaction(),
            self.summon_adult(),
            self.ketsgi(),
            self.henlo(),
            self.spongebob(),
            self.kool_aid(),
            self.parrot_wave(),
            self.bad_bot(),
            self.good_bot(),
        )?;

        Ok(())
    }

    async fn react_with_word(&self, word: &str) -> DiscordResult {
        for c in word.chars() {
            self.react_with(emojis::letter(c)).await?;
        }

        Ok(())
    }

    async fn react_with<R: Into<ReactionType> + Debug + Clone>(
        &self,
        reaction: R,
    ) -> DiscordResult {
        self.msg.react(&self.context.http, reaction.clone()).await
    }

    async fn send_message(&self, content: impl std::fmt::Display) -> DiscordResult {
        let _ = self.msg.channel_id.say(&self.context.http, content).await?;

        Ok(())
    }

    fn contains_emoji(&self, emoji: &Emoji) -> bool {
        self.msg.content.contains(&format!("{}", emoji))
    }

    fn sent_by(&self, user_id: UserId) -> bool {
        self.msg.author.id == user_id
    }

    async fn bepsi(&self) -> DiscordResult {
        if self.sent_by(users::aaron()) && thread_rng().gen_bool(0.15) {
            self.react_with(emojis::bepsi()).await?;
        }

        Ok(())
    }

    async fn gkappa(&self) -> DiscordResult {
        if thread_rng().gen_bool(0.0005) {
            self.react_with(emojis::gkappa()).await?;
        }

        Ok(())
    }

    async fn maga(&self) -> DiscordResult {
        if MAGA_RE.is_match(&self.msg.content) {
            self.react_with(emojis::maga()).await?;
        }

        Ok(())
    }

    async fn parrot_wave(&self) -> DiscordResult {
        if self.msg.content == "🦜" {
            let response = MessageBuilder::new()
                .push(format_emoji(&emojis::parrot_wave_7()))
                .push(" ")
                .push(format_emoji(&emojis::parrot_wave_6()))
                .push(" ")
                .push(format_emoji(&emojis::parrot_wave_5()))
                .push(" ")
                .push(format_emoji(&emojis::parrot_wave_4()))
                .push(" ")
                .push(format_emoji(&emojis::parrot_wave_3()))
                .push(" ")
                .push(format_emoji(&emojis::parrot_wave_2()))
                .push(" ")
                .push(format_emoji(&emojis::parrot_wave_1()))
                .build();

            self.send_message(response).await?;
        }

        Ok(())
    }

    async fn rizo_pls(&self) -> DiscordResult {
        if self.msg.mentions_user_id(users::rizo()) {
            self.react_with_word("rizopls").await?;
        }

        Ok(())
    }

    async fn sick(&self) -> DiscordResult {
        if self.msg.content == "🤢" {
            self.react_with_word("sick").await?;
        }

        Ok(())
    }

    async fn wendy_parrot(&self) -> DiscordResult {
        if self.sent_by(users::jerran()) && thread_rng().gen_bool(0.2) {
            self.react_with(emojis::wendy_parrot()).await?;
        }

        Ok(())
    }

    async fn gzack(&self) -> DiscordResult {
        if self.sent_by(users::zack()) && thread_rng().gen_bool(0.01) {
            self.react_with(emojis::gzack()).await?;
        }

        Ok(())
    }

    async fn zack(&self) -> DiscordResult {
        if self.sent_by(users::zack()) && thread_rng().gen_bool(0.2) {
            self.react_with(emojis::zack()).await?;
        }

        Ok(())
    }

    async fn trumpgasm(&self) -> DiscordResult {
        if self.msg.content.contains(&format!("{}", emojis::maga())) {
            self.react_with(emojis::trumpgasm()).await?;
        }

        Ok(())
    }

    async fn maga_react(&self) -> DiscordResult {
        if self.contains_emoji(&emojis::trumpgasm()) {
            self.react_with(emojis::maga()).await?;
        }

        Ok(())
    }

    async fn fuck_you(&self) -> DiscordResult {
        if self.msg.content == "fuck you all" {
            try_join!(
                self.fuck_aaron(),
                self.fuck_bacon(),
                self.fuck_jerran(),
                self.fuck_rizo(),
                self.fuck_zack(),
            )?;
        }

        if self.msg.content.contains("fuck you") {
            if self.msg.content.contains("aaron") || self.msg.mentions_user_id(users::aaron()) {
                self.fuck_aaron().await?;
            }

            if self.msg.content.contains("bacon") || self.msg.mentions_user_id(users::bacon()) {
                self.fuck_bacon().await?;
            }

            if self.msg.content.contains("jerran") || self.msg.mentions_user_id(users::jerran()) {
                self.fuck_jerran().await?;
            }

            if self.msg.content.contains("rizo") || self.msg.mentions_user_id(users::rizo()) {
                self.fuck_rizo().await?;
            }

            if self.msg.content.contains("zack") || self.msg.mentions_user_id(users::zack()) {
                self.fuck_zack().await?;
            }
        }

        Ok(())
    }

    async fn fuck_aaron(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .emoji(&emojis::bepsi())
            .push(" ")
            .mention(&users::aaron())
            .push(" 🖕")
            .build();

        self.send_message(response).await
    }
    async fn fuck_bacon(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .push("🥓 ")
            .mention(&users::bacon())
            .push(" 🖕")
            .build();

        self.send_message(response).await
    }
    async fn fuck_jerran(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .push(format_emoji(&emojis::wendy_parrot()))
            .push(" ")
            .mention(&users::jerran())
            .push(" 🖕")
            .build();

        self.send_message(response).await
    }
    async fn fuck_rizo(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .mention(&users::rizo())
            .push(" 🖕")
            .build();

        self.send_message(response).await
    }
    async fn fuck_zack(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .emoji(&emojis::zack())
            .push(" ")
            .mention(&users::zack())
            .push(" 🖕")
            .build();

        self.send_message(response).await
    }

    async fn steam(&self) -> DiscordResult {
        if self.sent_by(users::aaron()) && self.msg.content.contains("store.steampowered.com") {
            self.send_message("No Aaron, no one wants to buy your steam game")
                .await?;
        }

        Ok(())
    }

    async fn mention_bacon(&self) -> DiscordResult {
        if self.msg.content.contains("@🥓") {
            let response = MessageBuilder::new().mention(&users::bacon()).build();

            self.send_message(response).await?;
        }

        Ok(())
    }

    async fn mention_zack(&self) -> DiscordResult {
        if self.contains_emoji(&emojis::zack()) {
            let response = MessageBuilder::new().mention(&users::zack()).build();

            self.send_message(response).await?;
        }

        Ok(())
    }

    async fn nsa(&self) -> DiscordResult {
        if self.sent_by(users::rizo()) && self.msg.content.contains("alder") {
            self.send_message("👁👄👁 but the NSA").await?;
        }

        Ok(())
    }

    async fn random_rizo_reaction(&self) -> DiscordResult {
        let (chance, range) = {
            let mut rng = thread_rng();
            (rng.gen_bool(0.2), rng.gen_range(1, 4))
        };
        if self.sent_by(users::rizo()) && chance {
            match range {
                1 => {
                    self.react_with(emojis::parrot_wave_7()).await?;
                    self.react_with(emojis::parrot_wave_6()).await?;
                    self.react_with(emojis::parrot_wave_5()).await?;
                    self.react_with(emojis::parrot_wave_4()).await?;
                    self.react_with(emojis::parrot_wave_3()).await?;
                    self.react_with(emojis::parrot_wave_2()).await?;
                    self.react_with(emojis::parrot_wave_1()).await?;
                }
                2 => self.react_with(emojis::ultra_fast_parrot()).await?,
                _ => self.react_with(emojis::party_parrot()).await?,
            };
        }

        Ok(())
    }

    async fn summon_adult(&self) -> DiscordResult {
        if self.msg.content == "!adult" {
            let mention = MessageBuilder::new().role(&roles::adult()).build();
            let awful_face = MessageBuilder::new()
                .push_line("👁 👁")
                .push_line("       👄")
                .push_line("🤜  🤛")
                .build();

            self.send_message(mention).await?;
            self.send_message(awful_face).await?;
        }

        Ok(())
    }

    async fn ketsgi(&self) -> DiscordResult {
        let range = thread_rng().gen_range(0, 3);
        if self.msg.content.contains("letsgo") || self.msg.content.contains("ketsgi") {
            match range {
                0 => self.react_with_word("letsgo").await,
                1 => self.react_with_word("ketsgi").await,
                2 => self.react_with(emojis::lets_go()).await,
                _ => Err(serenity::Error::Other("Not possible to be here")),
            }?;
        }

        Ok(())
    }

    async fn henlo(&self) -> DiscordResult {
        if self.msg.content.contains("henlo") {
            let response = MessageBuilder::new()
                .push("henlo ")
                .mention(&self.msg.author)
                .push_line("")
                .push("helllo you STINKY ")
                .mention(&self.msg.author)
                .push_line("")
                .push("go shitpost ugly")
                .build();

            self.send_message(response).await?;
        }

        Ok(())
    }

    async fn spongebob(&self) -> DiscordResult {
        if self.msg.content.len() >= 10 && thread_rng().gen_bool(0.01) {
            let mut message_builder = MessageBuilder::new();

            for (i, c) in self.msg.content.chars().enumerate() {
                let p = if i % 2 == 0 {
                    c.to_ascii_lowercase()
                } else {
                    c.to_ascii_uppercase()
                };
                message_builder.push(p);
            }

            self.send_message(message_builder.build()).await?;
        }

        Ok(())
    }

    async fn kool_aid(&self) -> DiscordResult {
        if self.msg.content == "oh no" {
            try_join!(
                self.send_message(
                    "https://thumbs.gfycat.com/ImpartialFairDwarfrabbit-size_restricted.gif",
                ),
                self.react_with_word("ohyea"),
            )?;
        }

        Ok(())
    }

    async fn bad_bot(&self) -> DiscordResult {
        if self.msg.content == "bad bot" {
            let response = MessageBuilder::new()
                .push("fuck you ")
                .mention(&self.msg.author)
                .build();

            self.send_message(response).await?;
        }

        Ok(())
    }

    async fn good_bot(&self) -> DiscordResult {
        if self.msg.content == "good bot" {
            let response = MessageBuilder::new()
                .push(":blush: aww thanks ")
                .mention(&self.msg.author)
                .build();

            self.send_message(response).await?;
        }

        Ok(())
    }

    async fn friday(&self) -> DiscordResult {
        if self.contains_emoji(&emojis::friday()) {
            self.send_message("https://giphy.com/gifs/black-LqzgIzNWDiyFG")
                .await?;
        }

        Ok(())
    }

    async fn dog(&self) -> Result<(), serenity::Error> {
        let str_triggers = ["bark", "bork", "woof", "🐶"];
        if &self.msg.content == &format!("{}", emojis::wowee())
            || str_triggers.iter().any(|&x| x == self.msg.content)
        {
            let resp = reqwest::get("https://api.thedogapi.com/v1/images/search?mime_types=gif")
                .await?
                .json::<Vec<ApiResponse>>()
                .await?;

            self.send_message(&resp[0].url).await?;
        }

        Ok(())
    }
}
