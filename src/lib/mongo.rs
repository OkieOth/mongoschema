use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client };

pub fn connect_to_db() {
    let uri = "mongodb://admin:secretpassword@localhost:27017/admin";
    let client_options = ClientOptions::parse_async(uri).await?;
    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;

    // Send a ping to confirm a successful connection
    client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    for db in client.list_databases(None, None).await.unwrap() {
        println!("  found db: {}, size: {}", db.name, db.size_on_disk);
        let database = client.database(&db.name);
        let collections = database.list_collection_names(None).await.unwrap();

        // Print the names of all collections
        for collection in collections {
            println!("    Collection: {}", collection);
        }
    }
}
