use clap::Args;


#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Connection string to mongodb
    #[clap(long)]
    pub conn_str: String,

    /// Database to use
    #[clap(long)]
    pub db: String,

    /// Collections to parse
    #[clap(long)]
    pub collections: Vec<String>,

    /// Directory to write the created schemas in
    #[clap(long)]
    pub output_dir: String,

    /// If set to true, then in the output for every type will receive a dummy description field
    #[clap(long)]
    pub dummy_descr_types: bool,

    /// If set to true, then in the output for every attribute will receive a dummy description field
    #[clap(long)]
    pub dummy_descr_attrib: bool,

    /// If given and dummy_descr_* is set, then this text is printed there
    #[clap(long, default_value = "TODO")]
    pub dummy_descr_cont: String,
}


pub async fn create_schemas(args: CreateArgs) {
    println!("hello from the lib");
}

