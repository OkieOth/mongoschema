use log::{error, info, debug, LevelFilter};
use env_logger;
use env_logger::Env;

use clap::{Parser, Subcommand};

use mongoschema_impl::{CreateArgs, create_schemas};


/// Here's my app!
#[derive(Debug, Parser)]
#[clap(name = "mongoschema", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Use the program as a subscriber.
    Create(CreateArgs),
}


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let env = Env::default()
    .filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    let parser = App::parse();
    match parser.command {
        Command::Create(args) => {
            create_schemas(args).await;
        }
    };

    Ok(())
}