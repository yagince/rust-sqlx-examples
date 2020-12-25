use once_cell::sync::Lazy;

struct Config {
    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_database: String,
}

impl Config {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database
        )
    }
}

static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    postgres_host: std::env::var("POSTGRES_HOST").unwrap(),
    postgres_port: std::env::var("POSTGRES_PORT").unwrap(),
    postgres_user: std::env::var("POSTGRES_USER").unwrap(),
    postgres_password: std::env::var("POSTGRES_PASSWORD").unwrap(),
    postgres_database: std::env::var("POSTGRES_DB").unwrap(),
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // println!("{:?}", CONFIG.database_url());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&CONFIG.database_url())
        .await?;
    Ok(())
}
