use crate::{emojis, users};
use futures::try_join;
use rand::{thread_rng, Rng};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use serenity::{
    model::{
        channel::{Message, ReactionType},
        guild::Emoji,
        id::UserId,
    },
    prelude::*,
    utils::MessageBuilder,
};
use std::collections::HashMap;
use std::fmt::Debug;

lazy_static! {
    static ref BLAZE_IT_RE: Regex = Regex::new(r"4[./:]?20").unwrap();
    static ref PARROT_WAVES: Vec<Emoji> = vec![
        emojis::parrot_wave_7(),
        emojis::parrot_wave_6(),
        emojis::parrot_wave_5(),
        emojis::parrot_wave_4(),
        emojis::parrot_wave_3(),
        emojis::parrot_wave_2(),
        emojis::parrot_wave_1(),
    ];
    static ref DOG_TRIGGERS: [&'static str; 4] = ["bark", "bork", "woof", "🐶"];
    static ref HES_NOT_YOUR_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("friend", "buddy");
        m.insert("buddy", "guy");

        m
    };
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    url: String,
}

pub struct Reply {
    context: Context,
    msg: Message,
    client: Client,
}

type DiscordResult = Result<(), serenity::Error>;

impl Reply {
    pub fn new(context: Context, msg: Message) -> Reply {
        Reply {
            context,
            msg,
            client: Client::new(),
        }
    }

    pub async fn reply(&self) -> Result<(), Box<dyn std::error::Error>> {
        try_join!(
            // reaction replies
            self.wendy_parrot(),
            self.bepsi(),
            self.gkappa(),
            self.gzack(),
            self.zack(),
            self.rizo_pls(),
            self.sick(),
            self.friday(),
            self.dog(),
            self.the_architect(),
            self.blaze_it(),
            self.hes_not_your(),
            self.feels_bad_man(),
            self.malthor(),
            // simple replies
            self.fuck_you(),
            self.steam(),
            self.mention_bacon(),
            self.mention_zack(),
            self.nsa(),
            // unique replies
            self.random_rizo_reaction(),
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

    async fn react_with<R>(&self, reaction: R) -> DiscordResult
    where
        R: Into<ReactionType> + Debug + Clone,
    {
        self.msg.react(&self.context.http, reaction.clone()).await.and(Ok(()))
    }

    async fn send_message<T>(&self, content: T) -> DiscordResult
    where
        T: std::fmt::Display,
    {
        self.msg.channel_id.say(&self.context.http, content).await.and(Ok(()))
    }

    fn message_contains(&self, msg: impl Into<&'static str>) -> bool {
        self.msg.content.to_ascii_lowercase().contains(msg.into())
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

    async fn parrot_wave(&self) -> DiscordResult {
        if self.msg.content == "🦜" {
            let mut response = MessageBuilder::new();
            for wave in PARROT_WAVES.clone().into_iter() {
                response.push(wave).push(" ");
            }

            self.send_message(response.build()).await?;
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

        if self.message_contains("fuck you") {
            if self.msg.content.contains("aaron") || self.msg.mentions_user_id(users::aaron()) {
                self.fuck_aaron().await?;
            }

            if self.message_contains("bacon") || self.msg.mentions_user_id(users::bacon()) {
                self.fuck_bacon().await?;
            }

            if self.message_contains("jerran") || self.msg.mentions_user_id(users::jerran()) {
                self.fuck_jerran().await?;
            }

            if self.message_contains("rizo") || self.msg.mentions_user_id(users::rizo()) {
                self.fuck_rizo().await?;
            }

            if self.message_contains("zack") || self.msg.mentions_user_id(users::zack()) {
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
            .push(&emojis::wendy_parrot())
            .push(" ")
            .mention(&users::jerran())
            .push(" 🖕")
            .build();

        self.send_message(response).await
    }
    async fn fuck_rizo(&self) -> DiscordResult {
        let response = MessageBuilder::new().mention(&users::rizo()).push(" 🖕").build();

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
        if self.msg.content.contains(&format!("@{}", emojis::zack())) {
            let response = MessageBuilder::new().mention(&users::zack()).build();

            self.send_message(response).await?;
        }

        Ok(())
    }

    async fn nsa(&self) -> DiscordResult {
        if self.sent_by(users::rizo()) && self.message_contains("alder") {
            self.send_message("👁👄👁 but the NSA").await?;
        }

        Ok(())
    }

    async fn random_rizo_reaction(&self) -> DiscordResult {
        let (chance, range) = {
            let mut rng = thread_rng();
            (rng.gen_bool(0.2), rng.gen_range(1..=3))
        };
        if self.sent_by(users::rizo()) && chance {
            match range {
                1 => {
                    for wave in PARROT_WAVES.clone().into_iter() {
                        self.react_with(wave).await?;
                    }
                }
                2 => self.react_with(emojis::ultra_fast_parrot()).await?,
                _ => self.react_with(emojis::party_parrot()).await?,
            };
        }

        Ok(())
    }

    async fn ketsgi(&self) -> DiscordResult {
        let range = thread_rng().gen_range(0..=2);
        if self.message_contains("letsgo") || self.message_contains("ketsgi") {
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
        if self.message_contains("henlo") {
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
                self.send_message("https://thumbs.gfycat.com/ImpartialFairDwarfrabbit-size_restricted.gif",),
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
        let range = thread_rng().gen_range(0..=1);
        if self.contains_emoji(&emojis::friday()) {
            match range {
                0 => self.send_message("https://giphy.com/gifs/black-LqzgIzNWDiyFG").await,
                1 => self.send_message("https://www.youtube.com/watch?v=1AnG04qnLqI").await,
                _ => Err(serenity::Error::Other("Not possible to be here")),
            }?;
        }

        Ok(())
    }

    async fn dog(&self) -> DiscordResult {
        let is_wowee = self.msg.content == format!("{}", emojis::wowee());
        let is_dog_trigger = DOG_TRIGGERS.iter().any(|&x| self.message_contains(x));
        if is_wowee || is_dog_trigger {
            let resp = self
                .client
                .get("https://api.thedogapi.com/v1/images/search?mime_types=gif")
                .send()
                .await?
                .json::<Vec<ApiResponse>>()
                .await?;

            self.send_message(&resp[0].url).await?;
        }

        Ok(())
    }

    async fn the_architect(&self) -> DiscordResult {
        if self.message_contains("the architect") {
            let message = "_**T H E   A R C H I T E C T**_";

            self.send_message(message).await?;
        }

        Ok(())
    }

    async fn blaze_it(&self) -> DiscordResult {
        if self.message_contains("blaze it") || BLAZE_IT_RE.is_match(&self.msg.content) {
            self.react_with(emojis::snoop()).await?;
        }

        Ok(())
    }

    async fn hes_not_your(&self) -> DiscordResult {
        let trigger_word = HES_NOT_YOUR_MAP.keys().find(|&x| self.message_contains(*x));

        if let Some(trigger) = trigger_word {
            let output = HES_NOT_YOUR_MAP[trigger];
            self.send_message(format!("He's not your {}, {}", trigger, output))
                .await?;
        }

        Ok(())
    }

    async fn feels_bad_man(&self) -> DiscordResult {
        let fbm = emojis::feels_bad_man();
        if self.contains_emoji(&fbm) {
            self.react_with(fbm).await?;
        }
        Ok(())
    }

    async fn malthor(&self) -> DiscordResult {
        if self.message_contains("malthor") {
            self.send_message("You mean Malthor? The Destroyer?").await?;
        }
        Ok(())
    }
}
