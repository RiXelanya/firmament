use std::env;
use std::collections::HashMap ;
use dotenv::dotenv ;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use utils::dice::roll;

mod utils {
    pub mod dice;
}
mod parser ;
#[cfg(test)]
mod test ;

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    OracleInit(&'a str),
    OracleGuess(&'a str)
}

struct Handler;
struct OracleMap;
impl TypeMapKey for OracleMap {
    type Value = HashMap<u64, Vec<u8>>;
}

async fn execute(command : &Command<'_>, ctx: &Context, msg: &Message) {
    let name = msg.guild_id.unwrap();
    match command {
        Command::OracleInit(input) => {
            let result = roll(input); 
            let mut data = ctx.data.write().await;
            let map = data.get_mut::<OracleMap>().unwrap();
            let entry = map.entry(name.0).or_insert(result.clone());
            *entry = result ;
            if let Err(why) = msg.channel_id.say(&ctx.http, "You have initialized").await {
                println!("Error sending message: {:?}", why);
            }
        },
        Command::OracleGuess(input) => {
            let number : u8 = input.parse().unwrap();
            let data = ctx.data.read().await;
            let map = data.get::<OracleMap>().unwrap();
            let result = map.get(&name.0) ;
            match result {
                None => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "You have not initialized").await {
                    println!("Error sending message: {:?}", why);
                }},
                Some(c) => {
                    if c.contains(&number) {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Correct").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    else {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Wrong!").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let command: Option<Command> = parser::interpret(&msg.content);
        match command {
            Some(command) => execute(&command, &ctx, &msg).await ,
            None => {}
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler)
        .framework(StandardFramework::new())
        .await.expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<OracleMap>(HashMap::default());
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}