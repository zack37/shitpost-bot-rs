#[macro_use]
extern crate lazy_static;

use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use url::Url;

mod emojis;
mod reply;
mod roles;
mod users;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        let host = Url::parse(&msg.content);

        if msg.author.bot || host.is_ok() {
            return;
        }

        println!("{:#?}", msg);

        let mut reply = reply::Reply::new(context, msg);
        reply.reply();
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
