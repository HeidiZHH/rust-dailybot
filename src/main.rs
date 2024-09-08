use crate::store::Store;
use clap::Parser;
use teloxide::{prelude::*, utils::command::BotCommands};
mod store;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    let store = Store::new().unwrap();
    let parameters = ConfigParameters { store: store };

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(answer);

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![parameters])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(Clone)]
struct ConfigParameters {
    store: Store,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "adding new item.")]
    AddItem(String),
    #[command(description = "retrieve item.")]
    GetItem(String),
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    mut config: ConfigParameters,
) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => match config.store.set_user_name(username.clone()) {
            Ok(_) => {
                bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                    .await?
            }
            Err(err) => bot.send_message(msg.chat.id, err.to_string()).await?,
        },
        Command::AddItem(item) => match config.store.insert(item) {
            Ok(res) => {
                bot.send_message(msg.chat.id, format!("Item {} added.", res.get_id()))
                    .await?
            }
            Err(err) => bot.send_message(msg.chat.id, err.to_string()).await?,
        },
        Command::GetItem(id) => match config.store.get(id) {
            Ok(message) => bot.send_message(msg.chat.id, message).await?,
            Err(err) => bot.send_message(msg.chat.id, err.to_string()).await?,
        },
    };

    Ok(())
}
