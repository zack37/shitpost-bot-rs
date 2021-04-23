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
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    utils::MessageBuilder,
};
use url::Url;

struct Handler;

#[group]
#[commands(hype, adult, version)]
struct General;

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
    if let Err(_) = dotenv() {
        println!("no .env file found. Skipping");
    }
    env_logger::init();
    let token = env::var("DISCORD_TOKEN")?;
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    let _ = client.start().await?;

    Ok(())
}

async fn delete_message(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.delete(ctx).await {
        error!("{}", why);
    }

    Ok(())
}

#[command]
async fn hype(ctx: &Context, msg: &Message) -> CommandResult {
    delete_message(ctx, msg).await?;
    msg.channel_id
        .say(ctx, "https://media.giphy.com/media/b1o4elYH8Tqjm/giphy.gif")
        .await?;

    Ok(())
}

#[command]
async fn adult(ctx: &Context, msg: &Message) -> CommandResult {
    delete_message(ctx, msg).await?;
    let mention = MessageBuilder::new().role(&roles::adult()).build();
    let awful_face = MessageBuilder::new()
        .push_line("👁 👁")
        .push_line("       👄")
        .push_line("🤜  🤛")
        .build();

    msg.channel_id.say(&ctx, mention).await?;
    msg.channel_id.say(&ctx, awful_face).await?;

    Ok(())
}

#[command]
async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    delete_message(ctx, msg).await?;
    let message = format!(
        "You are running version {} of the shittiest posting bot",
        env!("CARGO_PKG_VERSION")
    );

    msg.channel_id.say(&ctx, message).await?;

    Ok(())
}

#[command]
async fn mufasa(ctx: &Context, msg: &Message) -> CommandResult {
    delete_message(ctx, msg).await?;
    msg.channel_id
        .say(&ctx, "https://www.youtube.com/watch?v=1AnG04qnLqI")
        .await?;

    Ok(())
}
