use serenity::model::id::{EmojiId, GuildId};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
#[macro_use]
extern crate lazy_static;

struct Handler;

lazy_static! {
    static ref SERVER_ID: GuildId = GuildId(std::env::args().nth(1).unwrap().parse().unwrap());
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if is_based(&message.content.to_lowercase()) {
            react(&ctx, &message, 748609686273523844);
        }
        if is_cringe(&message.content.to_lowercase()) {
            react(&ctx, &message, 748609686273523844);
        }
    }
}

fn react(ctx: &Context, msg: &Message, emoji: u64) {
    if let Err(e) = msg.react(ctx, EmojiId(emoji)) {
        println!("Could not react, error was: {:?}", e);
    }
}

fn is_cringe(s: &str) -> bool {
    s.contains("cringe")
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
