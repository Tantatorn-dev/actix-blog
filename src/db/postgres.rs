use tokio_postgres::NoTls;

pub struct Postgres {
    pub client: tokio_postgres::Client,
}

impl Postgres {
    pub async fn connect_to_db() -> Postgres {
        println!("Connecting to database");
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=lord_t password=password dbname=postgres", NoTls)
                .await
                .unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        return Postgres { client };
    }
}
