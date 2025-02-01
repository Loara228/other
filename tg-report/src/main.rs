use std::path::PathBuf;

use teloxide::{prelude::*, types::{InputFile, InputMediaPhoto}};
use clap::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Open {
        path: Option<PathBuf>
    },
    Purchase {
        path: Option<PathBuf>,

        #[arg(short, long)]
        lead: bool,
        
        #[arg(short, long)]
        text: String
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
                bot.send_photo(ChatId(1936488718i64),photo(path)).caption("Открытие").await.unwrap();
            },
            Commands::Purchase { path, lead, text } => {
                let mut lead_str = "Нет";
                if lead {
                    lead_str = "Да";
                }
                bot.send_photo(
                        ChatId(1936488718i64), 
                        photo(path))
                    .caption(format!("*Лид*: _{lead_str}_\n{}", text))
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2).await.unwrap();
            },
            _ => todo!("not implemented")
        }
    } else {
        println!("Try '{} --help' for more information", module_path!());
    }
}

fn photo(path: Option<PathBuf>) -> InputFile {
    let path = path.expect("input file is none");
    if !path.exists() {
        panic!("file not found")
    }
    InputFile::file(path)
}