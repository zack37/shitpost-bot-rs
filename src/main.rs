#![feature(async_closure)]
#![recursion_limit = "256"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod emojis;
mod reply;
mod roles;
mod users;

use std::env;

use dotenv::dotenv;
use reply::Reply;
use serenity::{
    async_trait,
    framework::standard::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use url::Url;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        let host = Url::parse(&msg.content);

        if msg.author.bot || host.is_ok() {
            return;
        }

        debug!("{:#?}", msg);

        let reply = Reply::new(context, msg);

        if let Err(why) = reply.reply().await {
            error!("{:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    env_logger::init();
    let token = env::var("DISCORD_TOKEN")?;
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await?;
    if let Err(why) = client.start().await {
        error!("Error with client: {:?}", why);
    }

    Ok(())
}
