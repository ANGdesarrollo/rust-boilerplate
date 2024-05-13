use std::time::Duration;
use sea_orm::{ ConnectOptions, Database, DatabaseConnection };
use log::info;
use std::env;

pub async fn init() -> Result<DatabaseConnection, sea_orm::DbErr> {
    dotenv::dotenv().ok();
    info!("Initializing DB");
    // TODO: Centralize env variables and validate them
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurada en el archivo .env");
    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("rust_experience");

    let db: DatabaseConnection = match Database::connect(opt).await {
        Ok(connection) => connection,
        Err(err) => return Err(err),
    };
    Ok(db)
}
