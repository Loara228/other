use clap::Parser;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

mod args;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            main_async().await;
        });
    
}

async fn main_async() {
    let args = args::Args::parse();
    let from: String = args.from.expect("missed arg: from");
    let password: String = args.password.expect("missed arg: password");
    let to: String = args.to.expect("missed arg: to");
    println!("from: {}, to: {}", from, to);
    if !ask_continue() {
        std::process::exit(0);
    }
    
}

fn ask_continue() -> bool {
    println!("continue? (Y/n)");
    let mut rk: String = String::new();
    std::io::stdin().read_line(&mut rk).unwrap();
    rk.trim().to_lowercase() == "y"
}