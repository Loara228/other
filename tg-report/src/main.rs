mod constants;

use std::path::PathBuf;

use chrono::Datelike;
use teloxide::{prelude::*, types::InputFile, utils::html::escape};
use clap::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(long, short)]
    text: Option<String>
}

#[derive(Subcommand)]
pub enum Commands {
    Open {
        path: PathBuf,
    },
    Purchase {
        path: PathBuf,
    },
    Close {
        path: PathBuf,

        #[arg(long, default_value = "0")]
        credit_requests: u32,

        #[arg(long, default_value = "0")]
        credit_responses: u32,

        #[arg(long, default_value = "0")]
        yasplit_requests: u32,

        #[arg(long, default_value = "0")]
        yasplit_responses: u32,

        #[arg(long, default_value = "0")]
        registrations: u32,

        #[arg(long, default_value = "0")]
        cash: u32,

        #[arg(long, default_value = "0")]
        adapter: u32
    }
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let cli = Cli::parse();

    if cli.command.is_some() {
        let bot = Bot::new(include_str!("../token.txt"));
        match cli.command.unwrap() {
            Commands::Open { path } => {
                bot.send_photo(ChatId(constants::DIALOG_ID), photo(path)).caption(cli.text.unwrap_or_default()).await.unwrap();
            },
            Commands::Close { path, credit_requests, credit_responses, yasplit_requests, yasplit_responses, registrations, cash, adapter } => {
                let msg = bot.send_photo(ChatId(constants::DIALOG_ID), photo(path));
                let date = chrono::Local::now().date_naive();
                let message_text = format!(
r#"_Отчет по закрытию {:02}/{:02}/{:04}_
МЦ *`U028 Радужный 7мкр12`*

1\. Заявки кд подано/выдано \- {}/{}
1\. Я сплит подано/выдано \- {}/{}
2\. Отзывы [Яндекс]({})/[ГИС]({}) \- 0/0
3\. Регистрация клиентов в 1С \- {}
4\. Адаптер шт \- {}
5\. Вклады \- 0
6\. Остаток дс в кассе \- _*{}₽*_
"#, date.day(), date.month(), date.year_ce().1, credit_requests, credit_responses, yasplit_requests, yasplit_responses, constants::MAPS_YANDEX_URL, {constants::MAPS_2GIS_URL}, registrations, adapter, cash);

                msg.parse_mode(teloxide::types::ParseMode::MarkdownV2).caption(escape(&message_text)).await.unwrap();
            }
            _ => todo!("not implemented")
        }
    } else {
        println!("Try '{} --help' for more information", module_path!());
    }
}

fn photo(path: PathBuf) -> InputFile {
    if !path.exists() {
        panic!("file not found")
    }
    InputFile::file(path)
}