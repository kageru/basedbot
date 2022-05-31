use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serenity::{
    model::{
        channel::ReactionType,
        id::{ChannelId, EmojiId, GuildId},
        prelude::*,
    },
    prelude::*,
    Client,
};

struct Handler;

lazy_static! {
    static ref SERVER_ID: GuildId = GuildId(std::env::args().nth(1).unwrap().parse().unwrap());
    static ref MEME_CHANNEL: ChannelId =
        ChannelId(std::env::args().nth(2).unwrap().parse().unwrap());
    static ref RETARD_REGEX: Regex = Regex::new("([^djDJh ])a( |$)").unwrap();
}

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
    // That other idiot who ends words with “a” instead of “er”
    if message.author.id == 261246789942902794 && RETARD_REGEX.is_match(&message.content) {
        message.delete(&ctx).await?;
        message
            .channel_id
            .say(&ctx, &format!("{} wollte sagen:", message.author.mention()))
            .await?;
        message
            .channel_id
            .say(
                &ctx,
                RETARD_REGEX
                    .replace_all(&message.content_safe(&ctx).await, |caps: &Captures| {
                        format!("{}**er**{}", &caps[1], &caps[2])
                    })
                    // some common false positives
                    .replace("etw**er**", "etwa")
                    .replace("europ**er**", "europa")
                    .replace("amerik**er**", "amerika")
                    .replace("chin**er**", "china")
                    .replace("mang**er**", "manga"),
            )
            .await?;
    }
    // that one idiot who always posts 5 links per message
    if message.author.id != 733488485813584012 && contains_video_link(&message.content) {
        message.channel_id.say(&ctx, "Working link:").await?;
        let fixed_link = message.content_safe(&ctx).await.replace(
            "https://media.discordapp.net/",
            "https://cdn.discordapp.com/",
        );
        message.channel_id.say(&ctx, fixed_link).await?;
    }
    if message.channel_id == *MEME_CHANNEL && is_meme(&message) {
        react(&ctx, &message, 748564944449962017, "based").await?;
        react(&ctx, &message, 748564944819060856, "cringe").await?;
    };
    let content = message.content_safe(&ctx).await.to_lowercase();
    if content.contains("everyone") && content.contains("nitro") && content.contains("http") {
        message.delete(&ctx).await?;
        message.channel_id.say(
            &ctx,
            &format!("{}: your message has been deleted because it triggered my spam filter. If you believe this to be in error, please contact the mods.", message.author.mention())
        ).await?;
    }
    Ok(())
}

fn contains_video_link(msg: &str) -> bool {
    msg.contains("https://media.discordapp.net/")
        && (msg.contains(".mp4") || msg.contains(".webm") || msg.contains(".mov"))
}

async fn react(
    ctx: &Context,
    msg: &Message,
    emoji: u64,
    name: &str,
) -> Result<(), serenity::Error> {
    let reaction = ReactionType::Custom {
        animated: false,
        id: EmojiId(emoji),
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
    let mut client =
        Client::builder(std::env::var("DISCORD_TOKEN").expect("no token in environment"))
            .event_handler(Handler)
            .await
            .expect("Could not create client");
    client.start().await.expect("could not start");
}
