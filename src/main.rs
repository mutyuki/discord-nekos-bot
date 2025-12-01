mod api;

use chrono::{Local, Timelike};
use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateMessage};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::ChannelId;
use std::env;
use std::time::Duration;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        tokio::spawn(async move {
            let _ = send_nightly_message(&ctx).await;

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

async fn send_nightly_message(ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let channel_id_str = env::var("DISCORD_CHANNEL_ID")?;
    let channel_id = ChannelId::new(channel_id_str.parse()?);

    let image_url = api::fetch_neko_image_url().await?;

    let image_bytes = api::fetch_image_bytes(&image_url).await?;

    let message_content = match api::generate_sleep_message(&image_bytes).await {
        Ok(msg) => msg,
        Err(_) => "まだ寝ないのかにゃ？".to_string(),
    };

    let attachment = CreateAttachment::bytes(image_bytes, "neko_image.jpg");

    channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(message_content)
                .add_file(attachment),
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

    let _ = client.start().await;
}
