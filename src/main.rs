use teloxide::{prelude2::*, utils::command::BotCommand};

use std::error::Error;
use std::process::Command;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum LampBotCommand { //possible commands
    #[command(description = "toggle lamp on")]
    On,
    #[command(description = "toggle lamp off")]
    Off,
}


async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: LampBotCommand,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        LampBotCommand::Off => { // handle off command by turning USB power off
            let command = Command::new("uhubctl")
            .arg("-l")
            .arg("1-1")
            .arg("-p")
            .arg("2")
            .arg("-a")
            .arg("0")
            .spawn()
            .expect("failed to execute process");
            command.stdout;
            bot.send_message(message.chat.id, format!("Lamp turned off!")).await?;
        },
        
        LampBotCommand::On => { // handle on command by turning USB power on
            let command = Command::new("uhubctl")
            .arg("-l")
            .arg("1-1")
            .arg("-p")
            .arg("2")
            .arg("-a")
            .arg("1")
            .spawn()
            .expect("failed to execute process");
            command.stdout;
            bot.send_message(message.chat.id, format!("Lamp turned on!")).await?;
        },
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting lamp_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::commands_repl(bot, answer, LampBotCommand::ty()).await;
}