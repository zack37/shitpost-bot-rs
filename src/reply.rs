use super::*;

use rand::{rngs::ThreadRng, thread_rng, Rng};
use regex::Regex;
use serde::Deserialize;
use serenity::{
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
    rng: ThreadRng,
}

type DiscordResult = Result<(), serenity::Error>;

impl Reply {
    pub fn new(context: Context, msg: Message) -> Reply {
        Reply {
            context,
            msg,
            rng: thread_rng(),
        }
    }

    pub fn reply(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // reaction replies
        self.wendy_parrot()?;
        self.bepsi()?;
        self.maga()?;
        self.gkappa()?;
        self.gzack()?;
        self.zack()?;
        self.trumpgasm()?;
        self.maga_react()?;
        self.rizo_pls()?;
        self.sick()?;
        self.friday()?;
        self.dog()?;

        // simple replies
        self.fuck_you()?;
        self.steam()?;
        self.mention_bacon()?;
        self.mention_zack()?;
        self.nsa()?;

        // unique replies
        self.random_rizo_reaction()?;
        self.summon_adult()?;
        self.ketsgi()?;
        self.henlo()?;
        self.spongebob()?;
        self.kool_aid()?;
        self.parrot_wave()?;
        self.bad_bot()?;
        self.good_bot()?;

        Ok(())
    }

    fn react_with_word(&self, word: &str) -> DiscordResult {
        for c in word.chars() {
            self.react_with(emojis::letter(c))?;
        }

        Ok(())
    }

    fn react_with<R: Into<ReactionType> + Debug + Clone>(&self, reaction: R) -> DiscordResult {
        self.msg.react(&self.context.http, reaction.clone()) 
    }

    fn send_message(&self, content: impl std::fmt::Display) -> DiscordResult {
        let _ = self.msg.channel_id.say(&self.context.http, content)?;

        Ok(())
    }

    fn contains_emoji(&self, emoji: &Emoji) -> bool {
        self.msg.content.contains(&format!("{}", emoji))
    }

    fn sent_by(&self, user_id: UserId) -> bool {
        self.msg.author.id == user_id
    }

    fn bepsi(&mut self) -> DiscordResult {
        if self.sent_by(users::aaron()) && self.rng.gen_bool(0.15) {
            self.react_with(emojis::bepsi())?;
        }

        Ok(())
    }

    fn gkappa(&mut self) -> DiscordResult {
        if self.rng.gen_bool(0.0005) {
            self.react_with(emojis::gkappa())?;
        }

        Ok(())
    }

    fn maga(&self) -> DiscordResult {
        if MAGA_RE.is_match(&self.msg.content) {
            self.react_with(emojis::maga())?;
        }

        Ok(())
    }

    fn parrot_wave(&self) -> DiscordResult {
        if self.msg.content == "🦜" {
            let response = MessageBuilder::new()
                .push(&emojis::parrot_wave_7())
                .push(" ")
                .push(&emojis::parrot_wave_6())
                .push(" ")
                .push(&emojis::parrot_wave_5())
                .push(" ")
                .push(&emojis::parrot_wave_4())
                .push(" ")
                .push(&emojis::parrot_wave_3())
                .push(" ")
                .push(&emojis::parrot_wave_2())
                .push(" ")
                .push(&emojis::parrot_wave_1())
                .build();

            self.send_message(response)?;
        }

        Ok(())
    }

    fn rizo_pls(&self) -> DiscordResult {
        if self.msg.mentions_user_id(users::rizo()) {
            self.react_with_word("rizopls")?;
        }

        Ok(())
    }

    fn sick(&self) -> DiscordResult {
        if self.msg.content == "🤢" {
            self.react_with_word("sick")?;
        }

        Ok(())
    }

    fn wendy_parrot(&mut self) -> DiscordResult {
        if self.sent_by(users::jerran()) && self.rng.gen_bool(0.2) {
            self.react_with(emojis::wendy_parrot())?;
        }

        Ok(())
    }

    fn gzack(&mut self) -> DiscordResult {
        if self.sent_by(users::zack()) && self.rng.gen_bool(0.01) {
            self.react_with(emojis::gzack())?;
        }

        Ok(())
    }

    fn zack(&mut self) -> DiscordResult {
        if self.sent_by(users::zack()) && self.rng.gen_bool(0.2) {
            self.react_with(emojis::zack())?;
        }

        Ok(())
    }

    fn trumpgasm(&mut self) -> DiscordResult {
        if self.msg.content.contains(&format!("{}", emojis::maga())) {
            self.react_with(emojis::trumpgasm())?;
        }

        Ok(())
    }

    fn maga_react(&mut self) -> DiscordResult {
        if self.contains_emoji(&emojis::trumpgasm()) {
            self.react_with(emojis::maga())?;
        }

        Ok(())
    }

    fn fuck_you(&self) -> DiscordResult {
        if self.msg.content == "fuck you all" {
            self.fuck_aaron()?;
            self.fuck_bacon()?;
            self.fuck_jerran()?;
            self.fuck_rizo()?;
            self.fuck_zack()?;
        }

        if self.msg.content.contains("fuck you") {
            if self.msg.content.contains("aaron") || self.msg.mentions_user_id(users::aaron()) {
                self.fuck_aaron()?;
            }

            if self.msg.content.contains("bacon") || self.msg.mentions_user_id(users::bacon()) {
                self.fuck_bacon()?;
            }

            if self.msg.content.contains("jerran") || self.msg.mentions_user_id(users::jerran()) {
                self.fuck_jerran()?;
            }

            if self.msg.content.contains("rizo") || self.msg.mentions_user_id(users::rizo()) {
                self.fuck_rizo()?;
            }

            if self.msg.content.contains("zack") || self.msg.mentions_user_id(users::zack()) {
                self.fuck_zack()?;
            }
        }

        Ok(())
    }

    fn fuck_aaron(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .emoji(&emojis::bepsi())
            .push(" ")
            .mention(&users::aaron())
            .push(" 🖕")
            .build();

        self.send_message(response)
    }
    fn fuck_bacon(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .push("🥓 ")
            .mention(&users::bacon())
            .push(" 🖕")
            .build();

        self.send_message(response)
    }
    fn fuck_jerran(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .push(&emojis::wendy_parrot())
            .push(" ")
            .mention(&users::jerran())
            .push(" 🖕")
            .build();

        self.send_message(response)
    }
    fn fuck_rizo(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .mention(&users::rizo())
            .push(" 🖕")
            .build();

        self.send_message(response)
    }
    fn fuck_zack(&self) -> DiscordResult {
        let response = MessageBuilder::new()
            .emoji(&emojis::zack())
            .push(" ")
            .mention(&users::zack())
            .push(" 🖕")
            .build();

        self.send_message(response)
    }

    fn steam(&self) -> DiscordResult {
        if self.sent_by(users::aaron()) && self.msg.content.contains("store.steampowered.com") {
            self.send_message("No Aaron, no one wants to buy your steam game")?;
        }

        Ok(())
    }

    fn mention_bacon(&self) -> DiscordResult {
        if self.msg.content.contains("@🥓") {
            let response = MessageBuilder::new().mention(&users::bacon()).build();

            self.send_message(response)?;
        }

        Ok(())
    }

    fn mention_zack(&self) -> DiscordResult {
        if self.contains_emoji(&emojis::zack()) {
            let response = MessageBuilder::new().mention(&users::zack()).build();

            self.send_message(response)?;
        }

        Ok(())
    }

    fn nsa(&self) -> DiscordResult {
        if self.sent_by(users::rizo()) && self.msg.content.contains("alder") {
            self.send_message("👁👄👁 but the NSA")?;
        }

        Ok(())
    }

    fn random_rizo_reaction(&mut self) -> DiscordResult {
        if self.sent_by(users::rizo()) && self.rng.gen_bool(0.2) {
            match self.rng.gen_range(1, 4) {
                1 => {
                    self.react_with(emojis::parrot_wave_7())?;
                    self.react_with(emojis::parrot_wave_6())?;
                    self.react_with(emojis::parrot_wave_5())?;
                    self.react_with(emojis::parrot_wave_4())?;
                    self.react_with(emojis::parrot_wave_3())?;
                    self.react_with(emojis::parrot_wave_2())?;
                    self.react_with(emojis::parrot_wave_1())?;
                }
                2 => self.react_with(emojis::ultra_fast_parrot())?,
                _ => self.react_with(emojis::party_parrot())?,
            };
        }

        Ok(())
    }

    fn summon_adult(&self) -> DiscordResult {
        if self.msg.content == "!adult" {
            let mention = MessageBuilder::new().role(&roles::adult()).build();
            let awful_face = MessageBuilder::new()
                .push_line("👁 👁")
                .push_line("       👄")
                .push_line("🤜  🤛")
                .build();

            self.send_message(mention)?;
            self.send_message(awful_face)?;
        }

        Ok(())
    }

    fn ketsgi(&mut self) -> DiscordResult {
        if self.msg.content.contains("letsgo") || self.msg.content.contains("ketsgi") {
            match self.rng.gen_range(0, 3) {
                0 => self.react_with_word("letsgo"),
                1 => self.react_with_word("ketsgi"),
                2 => self.react_with(emojis::lets_go()),
                _ => Err(serenity::Error::Other("Not possible to be here")),
            }?;
        }

        Ok(())
    }

    fn henlo(&self) -> DiscordResult {
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

            self.send_message(response)?;
        }

        Ok(())
    }

    fn spongebob(&mut self) -> DiscordResult {
        if self.msg.content.len() >= 10 && self.rng.gen_bool(0.01) {
            let mut message_builder = MessageBuilder::new();

            for (i, c) in self.msg.content.chars().enumerate() {
                let p = if i % 2 == 0 {
                    c.to_ascii_lowercase()
                } else {
                    c.to_ascii_uppercase()
                };
                message_builder.push(p);
            }

            self.send_message(message_builder.build())?;
        }

        Ok(())
    }

    fn kool_aid(&self) -> DiscordResult {
        if self.msg.content == "oh no" {
            self.send_message(
                "https://thumbs.gfycat.com/ImpartialFairDwarfrabbit-size_restricted.gif",
            )?;
            self.react_with_word("ohyea")?;
        }

        Ok(())
    }

    fn bad_bot(&self) -> DiscordResult {
        if self.msg.content == "bad bot" {
            let response = MessageBuilder::new()
                .push("fuck you ")
                .mention(&self.msg.author)
                .build();

            self.send_message(response)?;
        }

        Ok(())
    }

    fn good_bot(&self) -> DiscordResult {
        if self.msg.content == "good bot" {
            let response = MessageBuilder::new()
                .push(":blush: aww thanks ")
                .mention(&self.msg.author)
                .build();

            self.send_message(response)?;
        }

        Ok(())
    }

    fn friday(&self) -> DiscordResult {
        if self.contains_emoji(&emojis::friday()) {
            self.send_message("https://giphy.com/gifs/black-LqzgIzNWDiyFG")?
        }

        Ok(())
    }

    fn dog(&self) -> Result<(), serenity::Error> {
        let str_triggers = ["bark", "bork", "woof", "🐶"];
        if &self.msg.content == &format!("{}", emojis::wowee())
            || str_triggers.iter().any(|&x| x == self.msg.content)
        {
            let resp = reqwest::blocking::get(
                "https://api.thedogapi.com/v1/images/search?mime_types=gif",
            )?
            .json::<Vec<ApiResponse>>()?;

            self.send_message(&resp[0].url)?;
        }

        Ok(())
    }
}
