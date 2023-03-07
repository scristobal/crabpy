use std::sync::Arc;

use teloxide::{types::Message, utils::command::BotCommands, Bot};

use crate::{
    bot::{answer_cmd_repl, Command},
    requests, webhooks,
};

pub async fn run(
    bot: teloxide::Bot,
    connector: requests::Requests,
    webhooks_server: webhooks::WebhookServer,
) {
    tokio::spawn(webhooks_server.run());

    let connector = Arc::new(connector);

    teloxide::commands_repl(
        bot,
        move |bot: Bot, msg: Message, cmd: Command| {
            let connector = Arc::clone(&connector);

            async move { answer_cmd_repl(bot, msg, cmd, connector).await }
        },
        Command::ty(),
    )
    .await;
}
