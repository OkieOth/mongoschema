use clap::Args;
use crate::mongo::MongoClient;
use crate::utils::size_str;

#[derive(Debug, Args)]
pub struct ListDbsArgs {
    /// Connection string to mongodb
    #[clap(long)]
    pub conn_str: String,

    /// Prints more verbose output
    #[clap(long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct ListCollectionsArgs {
    /// Connection string to mongodb
    #[clap(long)]
    pub conn_str: String,

    /// Database to use
    #[clap(long)]
    pub db: String,

    /// Prints more verbose output
    #[clap(long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct ListIndexesArgs {
    /// Connection string to mongodb
    #[clap(long)]
    pub conn_str: String,

    /// Database to use
    #[clap(long)]
    pub db: String,

    /// Database to use
    #[clap(long)]
    pub collection: String,

    /// Prints more verbose output
    #[clap(long, default_value_t = false)]
    pub verbose: bool,
}

pub async fn list_dbs(args: ListDbsArgs) {
    let client = MongoClient::new(&args.conn_str).await;
    for d in client.client.list_databases(None, None).await.unwrap() {
        if args.verbose {
            let name = d.name;
            let size_on_disk = size_str(d.size_on_disk);
            println!("name: {}, size_on_disk: {}", name, size_on_disk);
        } else {
            println!("{}", d.name);
        }
    }

    println!("list_dbs is called");
}

pub async fn list_collections(args: ListCollectionsArgs) {
    println!("list_collections is called");
}

pub async fn list_indexes(args: ListIndexesArgs) {
    println!("list_indexes is called");
}
