#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

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

        debug!("{:#?}", msg);

        let mut reply = reply::Reply::new(context, msg);

        if let Err(why) = reply.reply() {
            error!("{:?}", why);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let token = env::var("DISCORD_TOKEN")?;
    let mut client = Client::new(&token, Handler)?;
    client.start()?;

    Ok(())
}
