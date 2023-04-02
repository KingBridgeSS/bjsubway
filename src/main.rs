#![allow(unused)]


mod net_dealer;
mod errors;
mod util;
mod core;

use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    action: Option<Action>,
    #[command[flatten]]
    detail: Detail,
    #[command[flatten]]
    find: Find,
}


#[derive(clap::Subcommand)]
enum Action {
    Update,
    Lines,
    Detail,
    Find,
}

#[derive(Args)]
#[command()]
struct Detail {
    #[arg(short, long, default_value = "")]
    lcode: String,
}

#[derive(Args)]
#[command()]
struct Find {
    #[arg(short, long, default_value = "")]
    from: String,
    #[arg(short, long, default_value = "")]
    to: String,
}


fn main() {
    let cli = Cli::parse();
    match &cli.action {
        Some(Action::Update) => {
            core::update();
        }
        Some(Action::Lines) => {
            core::show_lines();
        }
        Some(Action::Detail) => {
            core::print_detail(&cli.detail.lcode);
        }
        Some(Action::Find) => {
            core::find_lines(&cli.find.from, &cli.find.to);
        }
        None => {}
    }
}
