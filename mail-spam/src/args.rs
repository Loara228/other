use clap::Parser;


#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub from: Option<String>,

    #[arg(short, long)]
    pub password: Option<String>,

    #[arg(short, long)]
    pub to: Option<String>,

    #[arg(short, long)]
    pub message: Option<String>
}