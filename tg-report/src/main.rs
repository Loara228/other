mod constants;

use std::path::PathBuf;

use teloxide::{prelude::*, types::InputFile};
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
        let bot = Bot::new(include_str!("../target/token"));
        match cli.command.unwrap() {
            Commands::Open { path } => {
                bot.send_photo(ChatId(1936488718i64), photo(path)).caption(cli.text.unwrap_or_default()).await.unwrap();
            },
            Commands::Close { path, credit_requests, credit_responses, registrations, cash, adapter } => {
                let msg = bot.send_photo(ChatId(1936488718i64), photo(path));
                msg.caption(format!(
r#"Отчет по закрытию 
МЦ г. Радужный 7 мкр.
1. Заявки кд количество поданных/количество выданных - {}/{}
2. Отзывы Яндекс/ГИС - 0
3. Регистрация клиентов в 1С - {}
4. Адаптер шт - {}
5. Остаток дс в кассе - {}"#, credit_requests, credit_responses, registrations, adapter, cash)).await.unwrap();
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