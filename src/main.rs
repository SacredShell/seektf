use seektf::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Upcoming,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Upcoming => {
            upcoming().await.unwrap();
        }
    }
}