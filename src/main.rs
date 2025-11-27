use chrono::{Local, Timelike};
use serde::Deserialize;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::ChannelId;
use std::env;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct NekoResponse {
    url: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        tokio::spawn(async move {
            loop {
                let now = Local::now();
                if now.hour() == 23 && now.minute() == 0 {
                    let _ = send_nightly_message(&ctx).await;
                    tokio::time::sleep(Duration::from_secs(3600)).await;
                } else {
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }
            }
        });
    }
}

async fn fetch_neko_image() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://nekos.life/api/v2/img/neko")
        .await?
        .json::<NekoResponse>()
        .await?;
    Ok(response.url)
}

async fn send_nightly_message(ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let channel_id_str = env::var("DISCORD_CHANNEL_ID")?;
    let channel_id = ChannelId::new(channel_id_str.parse()?);
    let image_url = fetch_neko_image().await?;
    channel_id
        .send_message(
            &ctx.http,
            serenity::builder::CreateMessage::new()
                .content("そろそろ寝る時間ですにゃー🌙😴")
                .embed(
                    serenity::builder::CreateEmbed::new()
                        .image(image_url)
                        .color(0x9B59B6),
                ),
        )
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN が設定されていません");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Client の作成に失敗しました");
    if let Err(why) = client.start().await {
        eprintln!("Client エラー: {:?}", why);
    }
}
