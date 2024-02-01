use clap::Args;

#[derive(Debug, Args)]
pub struct ListDbsArgs {
    /// Connection string to mongodb
    #[clap(short, long)]
    pub conn_str: String,

    /// Prints more verbose output
    #[clap(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct ListCollectionsArgs {
    /// Connection string to mongodb
    #[clap(short, long)]
    pub conn_str: String,

    /// Database to use
    #[clap(short, long)]
    pub db: String,

    /// Prints more verbose output
    #[clap(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct ListIndexesArgs {
    /// Connection string to mongodb
    #[clap(short, long)]
    pub conn_str: String,

    /// Database to use
    #[clap(short, long)]
    pub db: String,

    /// Database to use
    #[clap(long)]
    pub collection: String,

    /// Prints more verbose output
    #[clap(short, long, default_value_t = false)]
    pub verbose: bool,
}

pub async fn list_dbs(args: ListDbsArgs) {
    println!("list_dbs is called");
}

pub async fn list_collections(args: ListCollectionsArgs) {
    println!("list_collections is called");
}

pub async fn list_indexes(args: ListIndexesArgs) {
    println!("list_indexes is called");
}
