mod commands;
mod config;
mod structs;
mod utils;

use commands::get_commands;
use config::{ApiKeys, AppConfig};
use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    keys: ApiKeys,
    reqwest: reqwest::Client,
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _state: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is ready!", data_about_bot.user.name)
        }

        _ => (),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let config = AppConfig::figment();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_commands().await,
            event_handler: |ctx, event, framework, state| {
                Box::pin(event_handler(ctx, event, framework, state))
            },
            ..Default::default()
        })
        .token(config.token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    keys: config.apis,
                    reqwest: reqwest::Client::new(),
                })
            })
        });

    framework.run().await.unwrap();
}
    