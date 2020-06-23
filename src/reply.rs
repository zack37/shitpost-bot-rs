use super::*;

use rand::{rngs::ThreadRng, thread_rng, Rng};
use regex::Regex;
use serenity::{
    model::{
        channel::{Message, ReactionType},
        id::UserId,
    },
    utils::MessageBuilder,
};
use std::fmt::Debug;

lazy_static! {
    static ref MAGA_RE: Regex = Regex::new("(?i)ma[dk]e .* great again").unwrap();
}

pub struct Reply {
    context: Context,
    msg: Message,
    rng: ThreadRng,
}

impl Reply {
    pub fn new(context: Context, msg: Message) -> Reply {
        Reply {
            context,
            msg,
            rng: thread_rng(),
        }
    }

    pub fn reply(&mut self) {
        // reaction replies
        self.wendy_parrot();
        self.bepsi();
        self.maga();
        self.gkappa();
        self.gzack();
        self.zack();
        self.trumpgasm();
        self.maga_react();
        self.rizo_pls();
        self.sick();

        // simple replies
        self.fuck_you();
        self.steam();
        self.mention_bacon();
        self.mention_zack();
        self.nsa();

        // unique replies
        self.random_rizo_reaction();
        self.summon_adult();
        self.ketsgi();
        self.henlo();
        self.spongebob();
        self.kool_aid();
        self.parrot_wave();
        self.bad_bot();
        self.good_bot();
    }

    fn react_with_word(&self, word: &str) {
        for c in word.chars() {
            self.react_with(emojis::letter(c));
        }
    }

    fn react_with<R: Into<ReactionType> + Debug + Clone>(&self, reaction: R) {
        if let Err(why) = self.msg.react(&self.context.http, reaction.clone()) {
            println!("Error sending message: {:?}\n{:?}", reaction, why);
            println!("original message: {}", self.msg.content);
        }
    }

    fn send_message(&self, content: impl std::fmt::Display) {
        if let Err(why) = self.msg.channel_id.say(&self.context.http, content) {
            println!("Error sending message: {:?}", why);
        }
    }

    fn sent_by(&self, user_id: UserId) -> bool {
        self.msg.author.id == user_id
    }

    fn bepsi(&mut self) {
        if self.sent_by(users::aaron()) && self.rng.gen_bool(0.15) {}
    }

    fn gkappa(&mut self) {
        if self.rng.gen_bool(0.0005) {
            self.react_with(emojis::gkappa());
        }
    }

    fn maga(&self) {
        if MAGA_RE.is_match(&self.msg.content) {
            self.react_with(emojis::maga());
        }
    }

    fn parrot_wave(&self) {
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

            self.send_message(response);
        }
    }

    fn rizo_pls(&self) {
        if self.msg.mentions_user_id(users::rizo()) {
            self.react_with_word("rizopls");
        }
    }

    fn sick(&self) {
        if self.msg.content == "🤢" {
            self.react_with_word("sick");
        }
    }

    fn wendy_parrot(&mut self) {
        if self.sent_by(users::jerran()) && self.rng.gen_bool(0.2) {
            self.react_with(emojis::wendy_parrot());
        }
    }

    fn gzack(&mut self) {
        if self.sent_by(users::zack()) && self.rng.gen_bool(0.01) {
            self.react_with(emojis::gzack());
        }
    }

    fn zack(&mut self) {
        if self.sent_by(users::zack()) && self.rng.gen_bool(0.2) {
            self.react_with(emojis::zack());
        }
    }

    fn trumpgasm(&mut self) {
        if self.msg.content.contains(&format!("{}", emojis::maga())) {
            self.react_with(emojis::trumpgasm());
        }
    }

    fn maga_react(&mut self) {
        if self
            .msg
            .content
            .contains(&format!("{}", emojis::trumpgasm()))
        {
            self.react_with(emojis::maga());
        }
    }

    fn fuck_you(&self) {
        if self.msg.content == "fuck you all" {
            self.fuck_aaron();
            self.fuck_bacon();
            self.fuck_jerran();
            self.fuck_rizo();
            self.fuck_zack();
        }

        if self.msg.content.contains("fuck you") {
            if self.msg.content.contains("aaron") || self.msg.mentions_user_id(users::aaron()) {
                self.fuck_aaron();
            }

            if self.msg.content.contains("bacon") || self.msg.mentions_user_id(users::bacon()) {
                self.fuck_bacon();
            }

            if self.msg.content.contains("jerran") || self.msg.mentions_user_id(users::jerran()) {
                self.fuck_jerran();
            }

            if self.msg.content.contains("rizo") || self.msg.mentions_user_id(users::rizo()) {
                self.fuck_rizo();
            }

            if self.msg.content.contains("zack") || self.msg.mentions_user_id(users::zack()) {
                self.fuck_zack();
            }
        }
    }

    fn fuck_aaron(&self) {
        let response = MessageBuilder::new()
            .emoji(&emojis::bepsi())
            .push(" ")
            .mention(&users::aaron())
            .push(" 🖕")
            .build();

        self.send_message(response);
    }
    fn fuck_bacon(&self) {
        let response = MessageBuilder::new()
            .push("🥓 ")
            .mention(&users::bacon())
            .push(" 🖕")
            .build();

        self.send_message(response);
    }
    fn fuck_jerran(&self) {
        let response = MessageBuilder::new()
            .push(&emojis::wendy_parrot())
            .push(" ")
            .mention(&users::jerran())
            .push(" 🖕")
            .build();

        self.send_message(response);
    }
    fn fuck_rizo(&self) {
        let response = MessageBuilder::new()
            .mention(&users::rizo())
            .push(" 🖕")
            .build();

        self.send_message(response);
    }
    fn fuck_zack(&self) {
        let response = MessageBuilder::new()
            .emoji(&emojis::zack())
            .push(" ")
            .mention(&users::zack())
            .push(" 🖕")
            .build();

        self.send_message(response);
    }

    fn steam(&self) {
        if self.sent_by(users::aaron()) && self.msg.content.contains("store.steampowered.com") {
            self.send_message("No Aaron, no one wants to buy your steam game");
        }
    }

    fn mention_bacon(&self) {
        if self.msg.content.contains("@🥓") {
            let response = MessageBuilder::new().mention(&users::bacon()).build();

            self.send_message(response);
        }
    }

    fn mention_zack(&self) {
        if self.msg.content.contains(&format!("{}", emojis::zack())) {
            let response = MessageBuilder::new().mention(&users::zack()).build();

            self.send_message(response);
        }
    }

    fn nsa(&self) {
        if self.sent_by(users::rizo()) && self.msg.content.contains("alder") {
            self.send_message("👁👄👁 but the NSA");
        }
    }

    fn random_rizo_reaction(&mut self) {
        if self.sent_by(users::rizo()) && self.rng.gen_bool(0.2) {
            match self.rng.gen_range(1, 4) {
                1 => {
                    self.react_with(emojis::parrot_wave_7());
                    self.react_with(emojis::parrot_wave_6());
                    self.react_with(emojis::parrot_wave_5());
                    self.react_with(emojis::parrot_wave_4());
                    self.react_with(emojis::parrot_wave_3());
                    self.react_with(emojis::parrot_wave_2());
                    self.react_with(emojis::parrot_wave_1());
                }
                2 => self.react_with(emojis::ultra_fast_parrot()),
                _ => self.react_with(emojis::party_parrot()),
            }
        }
    }

    fn summon_adult(&self) {
        if self.msg.content == "!adult" {
            let mention = MessageBuilder::new().role(&roles::adult()).build();
            let awful_face = MessageBuilder::new()
                .push_line("👁 👁")
                .push_line("       👄")
                .push_line("🤜  🤛")
                .build();

            self.send_message(mention);
            self.send_message(awful_face);
        }
    }

    fn ketsgi(&mut self) {
        if self.msg.content.contains("letsgo") || self.msg.content.contains("ketsgi") {
            match self.rng.gen_range(0, 3) {
                0 => self.react_with_word("letsgo"),
                1 => self.react_with_word("ketsgi"),
                2 => self.react_with(emojis::lets_go()),
                _ => println!("Not possible to be here"),
            };
        }
    }

    fn henlo(&self) {
        if self.msg.content.contains("henlo") {
            // let henlo_you = MessageBuilder::new().push("henlo ").mention(&self.msg.author).build();
            // let henlo_stinky = MessageBuilder::new().push("helllo you STINKY ").mention(&self.msg.author).build();
            // let go_shitpost = "go shitpost ugly";

            // self.send_message(henlo_you);
            // self.send_message(henlo_stinky);
            // self.send_message(go_shitpost);
            let response = MessageBuilder::new()
                .push("henlo ")
                .mention(&self.msg.author)
                .push_line("")
                .push("helllo you STINKY ")
                .mention(&self.msg.author)
                .push_line("")
                .push("go shitpost ugly")
                .build();

            self.send_message(response);
        }
    }

    fn spongebob(&mut self) {
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

            self.send_message(message_builder.build());
        }
    }

    fn kool_aid(&self) {
        if self.msg.content == "oh no" {
            self.send_message(
                "https://thumbs.gfycat.com/ImpartialFairDwarfrabbit-size_restricted.gif",
            );
            self.react_with_word("ohyea");
        }
    }

    fn bad_bot(&self) {
        if self.msg.content == "bad bot" {
            let response = MessageBuilder::new()
                .push("fuck you ")
                .mention(&self.msg.author)
                .build();

            self.send_message(response);
        }
    }

    fn good_bot(&self) {
        if self.msg.content == "good bot" {
            let response = MessageBuilder::new()
                .push(":blush: aww thanks ")
                .mention(&self.msg.author)
                .build();

            self.send_message(response);
        }
    }
}
