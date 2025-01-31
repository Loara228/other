use clap::Parser;
use mail_send::mail_builder::MessageBuilder;

mod args;

fn main() {
    let args = args::Args::parse();
    let from: String = args.from.expect("missed arg: from");
    let to: String = args.to.expect("missed arg: from");
    println!("from: {}, to: {}", from, to);
    if !ask_continue() {
        std::process::exit(0);
    }
    
    println!("hello world!")
}

fn ask_continue() -> bool {
    println!("continue? (Y/n)");
    let mut rk: String = String::new();
    std::io::stdin().read_line(&mut rk).unwrap();
    rk.trim().to_lowercase() == "y"
}