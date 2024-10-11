use async_trait::async_trait;
use serenity::{
    model::{
        channel::ReactionType,
        id::{ChannelId, EmojiId, GuildId},
        prelude::*,
    },
    prelude::*,
    Client,
};
use std::sync::LazyLock;

struct Handler;

static SERVER_ID: LazyLock<GuildId> =
    LazyLock::new(|| GuildId::new(std::env::args().nth(1).unwrap().parse().unwrap()));
static MEME_CHANNEL: LazyLock<ChannelId> =
    LazyLock::new(|| ChannelId::new(std::env::args().nth(2).unwrap().parse().unwrap()));

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if let Err(e) = handle_message(ctx, message).await {
            eprintln!("Error during command execution: {e}");
        }
    }
}

async fn handle_message(ctx: Context, message: Message) -> Result<(), serenity::Error> {
    if message.guild_id != Some(*SERVER_ID) {
        return Ok(());
    }
    if message.channel_id == *MEME_CHANNEL && is_meme(&message) {
        react(&ctx, &message, 748564944449962017, "based")
            .await
            .inspect_err(|e| eprintln!("Could not react to meme: {e}"))?;
        react(&ctx, &message, 748564944819060856, "cringe").await?;
    };
    let content = message.content_safe(&ctx).to_lowercase();
    if content.contains("everyone") && content.contains("nitro") && content.contains("http") {
        println!(
            "Deleting probable spam from user {}: “{}”",
            message.author.name, content
        );
        message.delete(&ctx).await?;
        message.channel_id.say(
            &ctx,
            &format!("{}: your message has been deleted because it triggered my spam filter. If you believe this to be in error, please contact the mods.", message.author.mention())
        ).await?;
    }
    Ok(())
}

async fn react(
    ctx: &Context,
    msg: &Message,
    emoji: u64,
    name: &str,
) -> Result<(), serenity::Error> {
    let reaction = ReactionType::Custom {
        animated: false,
        id: EmojiId::new(emoji),
        name: Some(name.to_string()),
    };
    msg.react(ctx, reaction).await?;
    Ok(())
}

fn is_meme(msg: &Message) -> bool {
    !msg.attachments.is_empty() || msg.content.to_lowercase().contains("http")
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(
        std::env::var("DISCORD_TOKEN").expect("no token in environment"),
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .expect("Could not create client");
    client.start().await.expect("could not start");
}
