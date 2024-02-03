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

async fn list_dbs_impl(args: ListDbsArgs) -> Result<Vec<String>, String> {
    let client = MongoClient::new(&args.conn_str).await;
    let mut ret: Vec<String> = Vec::new();
    match client.client.list_databases(None, None).await {
        Ok(dbs) => {
            for d in dbs {
                if args.verbose {
                    let size_on_disk = size_str(d.size_on_disk);
                    ret.push(format!("name: {}, size_on_disk: {}", d.name, size_on_disk));
                } else {
                    ret.push(format!("{}", d.name));
                }
            }
        },
        Err(e) => return Err(e.to_string()),
    };
    Ok(ret)
}

pub async fn list_dbs(args: ListDbsArgs) {
    if let Ok(text_vec) = list_dbs_impl(args).await {
        text_vec.iter().for_each(|s| println!("{}", s));
    }
}

pub async fn list_collections(args: ListCollectionsArgs) {
    println!("list_collections is called");
}

pub async fn list_indexes(args: ListIndexesArgs) {
    println!("list_indexes is called");
}

mod test {
    use crate::ListDbsArgs;

    use super::list_dbs_impl;
    use std::env;

    #[test]
    #[ignore]
    fn test_list_dbs_impl() {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let conn_str = match env::var("MONGO_CONSTR") {
                Ok(s) => s,
                Err(_) => "mongodb://admin:secretpassword@localhost:27017/admin".to_string(),
            };
            println!("test_connection: constr={}", conn_str);
            let args = ListDbsArgs {
                conn_str,
                verbose: false,
            };
            let dbs_vec = list_dbs_impl(args).await.unwrap();
            assert_eq!(dbs_vec.len(), 4);
            assert_eq!(dbs_vec[0], "admin".to_string());
            assert_eq!(dbs_vec[1], "config".to_string());
            assert_eq!(dbs_vec[2], "dummy".to_string());
            assert_eq!(dbs_vec[3], "local".to_string());
        });
    }

    #[test]
    #[ignore]
    fn test_list_dbs_impl2() {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let conn_str = match env::var("MONGO_CONSTR") {
                Ok(s) => s,
                Err(_) => "mongodb://admin:secretpassword@localhost:27017/admin".to_string(),
            };
            println!("test_connection: constr={}", conn_str);
            let args = ListDbsArgs {
                conn_str,
                verbose: true,
            };
            let dbs_vec = list_dbs_impl(args).await.unwrap();
            assert_eq!(dbs_vec.len(), 4);
            assert!(dbs_vec[0].starts_with("name: admin"));
            assert!(dbs_vec[0].ends_with(" kB"));
            assert!(dbs_vec[1].starts_with("name: config"));
            assert!(dbs_vec[1].ends_with(" kB"));
            assert!(dbs_vec[2].starts_with("name: dummy"));
            assert!(dbs_vec[2].ends_with(" kB"));
            assert!(dbs_vec[3].starts_with("name: local"));
            assert!(dbs_vec[3].ends_with(" kB"));
        });
    }

}