use serenity::model::channel::ReactionType;
use serenity::model::id::{ChannelId, EmojiId, GuildId};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
#[macro_use]
extern crate lazy_static;

struct Handler;

lazy_static! {
    static ref SERVER_ID: GuildId = GuildId(std::env::args().nth(1).unwrap().parse().unwrap());
    static ref MEME_CHANNEL: ChannelId =
        ChannelId(std::env::args().nth(2).unwrap().parse().unwrap());
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if message.guild_id != Some(*SERVER_ID) {
            return;
        }
        if message.content.contains("https://media.discordapp.net/")
            && (message.content.contains(".mp4")
                || message.content.contains(".webm")
                || message.content.contains(".mov"))
        {
            if let Err(e) = message.channel_id.say(&ctx, "Working link:") {
                eprint!("Could not send fixed link: {:?}", e);
            }
            if let Err(e) = message.channel_id.say(
                &ctx,
                message.content_safe(&ctx).replace(
                    "https://media.discordapp.net/",
                    "https://cdn.discordapp.com/",
                ),
            ) {
                eprint!("Could not send fixed link: {:?}", e);
            };
        }
        if message.channel_id == *MEME_CHANNEL && is_meme(&message) {
            react(&ctx, &message, 748564944449962017, "based");
            react(&ctx, &message, 748564944819060856, "cringe");
        }
        let content = message.content.to_lowercase();
        if content.contains("@everyone") && content.contains("nitro") && content.contains("http") {
            if let Err(e) = message.delete(&ctx) {
                eprint!("Could not delete spam: {}", e);
            }
            if let Err(e) = message.channel_id.say(
                &ctx,
                &format!("{}: your message has been deleted because it triggered my spam filter. If you believe this to be in error, please contact the mods.", message.author.mention())
            ) {
                eprint!("Could not respond to spam: {}", e);
            }
        }
    }
}

fn react(ctx: &Context, msg: &Message, emoji: u64, name: &str) {
    let reaction = ReactionType::Custom {
        animated: false,
        id: EmojiId(emoji),
        name: Some(name.to_string()),
    };
    if let Err(e) = msg.react(ctx, reaction) {
        println!("Could not react, error was: {:?}", e);
    }
}

fn is_meme(msg: &Message) -> bool {
    !msg.attachments.is_empty() || msg.content.to_lowercase().contains("http")
}

pub fn main() {
    let mut client = Client::new(
        std::env::var("DISCORD_TOKEN").expect("no token in environment"),
        Handler,
    )
    .expect("Could not create client");
    client.start().expect("could not start");
}
