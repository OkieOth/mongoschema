use clap::Args;

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Connection string to mongodb
    #[clap(short, long)]
    pub conn_str: String,

    /// Database to use
    #[clap(short, long)]
    pub db: String,

    /// Collections to parse
    #[clap(short, long)]
    pub collections: Vec<String>,

    /// Directory to write the created schemas in. If not given, then the output is to stdout
    #[clap(short, long)]
    pub output_dir: Option<String>,

    /// If set to true, then in the output for every type will receive a dummy description field
    #[clap(long, default_value_t = false)]
    pub dummy_descr_types: bool,

    /// If set to true, then in the output for every attribute will receive a dummy description field
    #[clap(long, default_value_t = false)]
    pub dummy_descr_attrib: bool,

    /// If given and dummy_descr_* is set, then this text is printed there
    #[clap(long, default_value = "TODO")]
    pub dummy_descr_cont: String,
}

pub async fn create_schemas(args: CreateArgs) {
    println!("create_schemas is called");
}
