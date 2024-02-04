use std::collections::binary_heap::Iter;

use clap::Args;
use mongodb::options::SelectionCriteria;
use mongodb::results::CollectionSpecification;
use mongodb::IndexModel;
use mongodb::{options::CollectionOptions, Collection, bson::doc};
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

    /// Prints more verbose output. Doesn't show additional information.
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
    match client.client.list_databases(None, None).await {
        Ok(dbs) => {
            let mut ret: Vec<String> = Vec::new();
            for d in dbs {
                if args.verbose {
                    let size_on_disk = size_str(d.size_on_disk);
                    ret.push(format!("name: {}, size_on_disk: {}", d.name, size_on_disk));
                } else {
                    ret.push(format!("{}", d.name));
                }
            }
            return Ok(ret);
        },
        Err(e) => return Err(e.to_string()),
    };
}

pub async fn list_dbs(args: ListDbsArgs) {
    match list_dbs_impl(args).await {
        Ok(text_vec) =>text_vec.iter().for_each(|s| println!("{}", s)),
        Err(e) => println!("\nerror while query databases\n{}", e),
    }
}

pub async fn list_collections_impl(args: ListCollectionsArgs) -> Result<Vec<String>, String> {
    let client = MongoClient::new(&args.conn_str).await;

    if let Err(msg) = client.ping(&args.db).await {
        return Err(msg);
    }

    let db = client.client.database(&args.db);

    match db.list_collection_names(None).await {
        Ok(collection_names) => {
            let mut ret: Vec<String> = Vec::new();
            for name in collection_names {
                if args.verbose {
                    ret.push(format!("name: {}", name));
                } else {
                    ret.push(name);
                }
            };
            return Ok(ret);
        },
        Err(e) => {
            return Err(e.to_string());
        },
    };
}

pub async fn list_collections(args: ListCollectionsArgs) {
    match list_collections_impl(args).await {
        Ok(text_vec) =>text_vec.iter().for_each(|s| println!("{}", s)),
        Err(e) => println!("\nerror while query collections\n{}", e),
    }
}

pub async fn list_indexes_impl(args: ListIndexesArgs) -> Result<Vec<String>, String> {
    let client = MongoClient::new(&args.conn_str).await;

    if let Err(msg) = client.ping(&args.db).await {
        return Err(msg);
    }

    let db = client.client.database(&args.db);

    let c: Collection<()> = db.collection(&args.collection);
    if let Ok(indexes) = c.list_indexes(None).await.as_mut() {
        while let Ok(b) = indexes.advance().await {
            if ! b {
                break;
            }
            let i = indexes.current();
            println!("{}", i.get)
        }
    }
    todo!();
}

pub async fn list_indexes(args: ListIndexesArgs) {
    match list_indexes_impl(args).await {
        Ok(text_vec) =>text_vec.iter().for_each(|s| println!("{}", s)),
        Err(e) => println!("\nerror while query indexes\n{}", e),
    }
}

mod test {
    use crate::{ListDbsArgs, ListCollectionsArgs};

    use super::{list_dbs_impl, list_collections_impl};
    use std::env;


    #[test]
    #[ignore]
    fn test_list_collections_impl() {
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
            let args = ListCollectionsArgs {
                conn_str,
                db: "dummy".to_string(),
                verbose: false,
            };
            let dbs_vec = list_collections_impl(args).await.unwrap();
            assert_eq!(dbs_vec.len(), 2);
            assert_eq!(dbs_vec[0], "c2".to_string());
            assert_eq!(dbs_vec[1], "c1".to_string());
        });
    }

    #[test]
    #[ignore]
    fn test_list_collections_constr() {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let conn_str = match env::var("MONGO_CONSTR") {
                Ok(s) => s,
                Err(_) => "mongodb://admin:secretpassword@localhost:27018/admin".to_string(),
            };
            println!("test_connection: constr={}", conn_str);
            let args = ListCollectionsArgs {
                conn_str,
                db: "dummy".to_string(),
                verbose: false,
            };
            assert!(list_collections_impl(args).await.is_err());
        });
    }

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