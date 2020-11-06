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
    static ref DELET_THIS_ID: EmojiId = EmojiId(323238442869719040);
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if message.guild_id != Some(*SERVER_ID) {
            return;
        }
        if message.channel_id == *MEME_CHANNEL && is_meme(&message) {
            react(&ctx, &message, 748564944449962017, "based");
            react(&ctx, &message, 748564944819060856, "cringe");
            return;
        }
        if is_based(&message.content.to_lowercase()) {
            react(&ctx, &message, 748564944449962017, "based");
        }
        if is_cringe(&message.content.to_lowercase()) {
            react(&ctx, &message, 748564944819060856, "cringe");
        }
    }

    fn reaction_add(&self, ctx: Context, added_reaction: Reaction) {
        if is_delet_this(&added_reaction.emoji) {
            let msg = added_reaction.message(&ctx).unwrap();
            if msg.channel_id != *MEME_CHANNEL
                && is_meme(&msg)
                && msg
                    .reactions
                    .iter()
                    .find(|r| is_delet_this(&r.reaction_type))
                    .map(|r| r.count >= 5)
                    .unwrap_or(false)
            {
                println!(
                    "Deleting message {:?} with content {} and attachments {:?}",
                    msg, msg.content, msg.attachments
                );
                if let Err(e) = msg.delete(&ctx) {
                    println!("Could not delete, error was: {:?}", e);
                }
            }
        }
    }
}

fn is_delet_this(t: &ReactionType) -> bool {
    match t {
        ReactionType::Custom {
            animated: _,
            id,
            name: _,
        } => id == &*DELET_THIS_ID,
        _ => false,
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

fn is_cringe(s: &str) -> bool {
    s.contains("cringe") || s.contains("cringy")
}

fn is_based(s: &str) -> bool {
    (s.contains("based") || s.contains("basiert")) && !s.contains("based on")
}

pub fn main() {
    let mut client = Client::new(
        std::env::var("DISCORD_TOKEN").expect("no token in environment"),
        Handler,
    )
    .expect("Could not create client");
    client.start().expect("could not start");
}
