use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client };

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MongoClient {
    pub client: Client,
}

impl MongoClient {
    pub async fn new(con_str: &str) -> MongoClient {
        let client_options = ClientOptions::parse_async(con_str).await.unwrap();
        // Create a new client and connect to the server
        let client = Client::with_options(client_options).unwrap();
        MongoClient {
            client,
        }
    }

    pub async fn ping(&self, db: &str) -> Result<(), String> {
        match self.client.database(db).run_command(doc! { "ping": 1 }, None).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }
}


mod test {
    use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client };
    use std::env;


    #[test]
    #[ignore]
    fn test_connection() {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let con_str = match env::var("MONGO_CONSTR") {
                Ok(s) => s,
                Err(_) => "mongodb://admin:secretpassword@localhost:27017/admin".to_string(),
            };
            println!("test_connection: constr={}", con_str);
            let client_options = ClientOptions::parse_async(con_str).await.unwrap();
            // Create a new client and connect to the server
            let client = Client::with_options(client_options).unwrap();
            // Send a ping to confirm a successful connection
            client.database("admin").run_command(doc! { "ping": 1 }, None).await.unwrap();
            println!("Pinged the database. You successfully connected to MongoDB!");
        });
    }
}