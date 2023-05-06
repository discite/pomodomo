use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!pomodomo " {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Starting pomodoro...").await {
                error!("Error sending message: {:?}", e);
            }
            let work_duration = 25; // in minutes
            let short_break_duration = 5; // in minutes
            let long_break_duration = 15; // in minutes
            let num_pomodoros_until_long_break = 4;

            for i in 0..num_pomodoros_until_long_break {
                // Work period
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("Starting work period {}...", i + 1))
                    .await
                {
                    error!("Error sending message: {:?}", e);
                }
                sleep(Duration::from_secs(work_duration * 60)).await;

                // Short break
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, "Taking a short break...")
                    .await
                {
                    error!("Error sending message: {:?}", e);
                }
                sleep(Duration::from_secs(short_break_duration * 60)).await;
            }

            // Long break
            if let Err(e) = msg
                .channel_id
                .say(&ctx.http, "Taking a long break...")
                .await
            {
                error!("Error sending message: {:?}", e);
            }
            sleep(Duration::from_secs(long_break_duration * 60)).await;

            // Pomodoro session complete
            if let Err(e) = msg
                .channel_id
                .say(&ctx.http, "Pomodoro session complete!")
                .await
            {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
