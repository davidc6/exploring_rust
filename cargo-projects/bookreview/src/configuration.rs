use config::{Config, File};

pub fn configuration() -> Result<Settings, config::ConfigError> {
    let builder = Config::builder()
        .add_source(File::with_name("configuration"));

    builder.build().unwrap().try_deserialize::<Settings>()
}

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DbSettings,
    pub app_port: u16
}

#[derive(serde::Deserialize, Debug)]
pub struct DbSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DbSettings {
    // postgres://${DB_USER}:${DB_PASSWORD}@{DB_HOST}:${DB_PORT}/${DB_NAME}
    pub fn conn_str(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }

    pub fn conn_str_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}