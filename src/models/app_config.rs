pub struct AppConfig {
    pub database_url: String,
    pub database_pool_size: u32,
}

impl AppConfig {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_pool_size = std::env::var("DATABASE_POOL_SIZE")
            .expect("DATABASE_POOL_SIZE must be set")
            .parse()
            .expect("DATABASE_POOL_SIZE must be a number");
        Self {
            database_url,
            database_pool_size,
        }
    }
}
