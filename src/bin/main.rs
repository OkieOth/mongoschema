use log::{error, info, debug, LevelFilter};
use env_logger;
use env_logger::Env;

use clap::{Parser, Subcommand};

use mongoschema_impl::{create_schemas, list_collections, list_dbs, list_indexes, CreateArgs, ListCollectionsArgs, ListDbsArgs, ListIndexesArgs, MongoClient};


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
    /// List all databases in to that connection.
    Dbs(ListDbsArgs),
    /// List all collections in a given database.
    Collections(ListCollectionsArgs),
    /// List all indexes for a given collection of a database.
    Indexes(ListIndexesArgs),
}


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let env = Env::default()
    .filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    let parser = App::parse();
    match parser.command {
        Command::Create(args) => create_schemas(args).await,
        Command::Dbs(args) => list_dbs(args).await,
        Command::Collections(args) => list_collections(args).await,
        Command::Indexes(args) => list_indexes(args).await,
    };

    Ok(())
}