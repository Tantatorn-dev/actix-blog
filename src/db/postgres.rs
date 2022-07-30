use tokio_postgres::{Error, NoTls};

pub async fn connect_to_db() -> Result<tokio_postgres::Client, Error> {
    println!("Connecting to database");
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=lord_t password=password", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e); 
        }
    });

    Ok(client)
}
