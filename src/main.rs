use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

mod leaderboard;

#[tokio::main]
async fn main() {
    leaderboard::test().await;
    println!("Hello, world!");
}
