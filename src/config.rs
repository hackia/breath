use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BreathConfig {
    pub breathes: Config,
    pub documentation: Documentation,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Documentation {
    pub doc: Vec<String>,
    pub man: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub scopes: Vec<String>,
    pub types: Vec<String>,
}

pub fn load_config() -> BreathConfig {
    let content = std::fs::read_to_string("breath.yml").expect("failed to read breath.yml");
    serde_yaml::from_str(&content).expect("syntax error in breath.yml")
}
pub fn init_config() -> Result<(), serde_yaml::Error> {
    let config = BreathConfig {
        breathes: Config {
            scopes: vec![],
            types: vec![
                String::from("feat"),
                String::from("chore"),
                String::from("fix"),
                String::from("docs"),
                String::from("style"),
                String::from("refactor"),
                String::from("perf"),
                String::from("test"),
                String::from("build"),
                String::from("ci"),
                String::from("revert"),
                String::from("release"),
                String::from("bump"),
            ],
        },
        documentation: Documentation {
            doc: vec![],
            man: vec![],
        },
    };
    let config = serde_yaml::to_string(&config)?;
    let config_path = std::path::Path::new("breath.yml");
    let mut f = File::create(config_path).expect("failed to create breath.yml");
    f.write_all(config.as_bytes())
        .expect("failed to write breath.yml");
    f.sync_all().expect("failed to sync breath.yml");
    Ok(())
}
