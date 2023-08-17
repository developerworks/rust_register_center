use serde::Deserialize;
use tokio_fs::File;

#[derive(Deserialize)]
struct Config {
    endpoints: Vec<String>,
}

pub async fn load_config() -> Config {
    let mut f = File::open("config.toml").await?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    toml::from_str(&contents)?
}
