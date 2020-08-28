use serenity::model::id::GuildId;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use std::sync::Mutex;
use std::time::{Duration, Instant};
#[macro_use]
extern crate lazy_static;

struct Handler;

static BASED_URL: &str =
    "https://cdn.discordapp.com/attachments/246368272327507979/738083492474257538/based.jpg";
static CRINGE_URL: &str =
    "https://cdn.discordapp.com/attachments/246368272327507979/738083495162806282/cringe.jpg";
static BOTH_URL: &str =
    "https://cdn.discordapp.com/attachments/300723600632053761/739835069417390110/based.jpg";
static COOLDOWN: Duration = Duration::from_secs(3600);

lazy_static! {
    static ref LAST_SENT: Mutex<Instant> = Mutex::new(Instant::now() - COOLDOWN);
    static ref SERVER_ID: GuildId = GuildId(std::env::args().nth(1).unwrap().parse().unwrap());
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if let Some(msg) = optional_reply(&message) {
            if let Err(e) = message.channel_id.say(&ctx, msg) {
                println!("Could not send image, error was: {:?}", e);
            }
        }
    }
}

fn is_cringe(s: &str) -> bool {
    s.contains("cringe")
}

fn is_based(s: &str) -> bool {
    (s.contains("based") || s.contains("basiert")) && !s.contains("based on")
}

fn optional_reply(msg: &Message) -> Option<&str> {
    if msg.guild_id == Some(*SERVER_ID) {
        let url = match msg.content.to_lowercase() {
            c if is_cringe(&c) && is_based(&c) => BOTH_URL,
            c if is_based(&c) => BASED_URL,
            c if is_cringe(&c) => CRINGE_URL,
            _ => return None,
        };
        let mut last = LAST_SENT.lock().unwrap();
        if last.elapsed() > COOLDOWN {
            println!("Sending image at {:?}", Instant::now());
            *last = Instant::now();
            return Some(url);
        }
    }
    None
}

pub fn main() {
    let mut client = Client::new(
        std::env::var("DISCORD_TOKEN").expect("no token in environment"),
        Handler,
    )
    .expect("Could not create client");
    client.start().expect("could not start");
}
