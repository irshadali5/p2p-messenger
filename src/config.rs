//! Application configuration.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub node_name: String,
    pub data_dir: std::path::PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node_name: "p2p-messenger".to_string(),
            data_dir: directories::ProjectDirs::from("com", "p2p-messenger", "p2p-messenger")
                .map(|d| d.data_dir().to_path_buf())
                .unwrap_or_else(|| std::path::PathBuf::from("./data")),
        }
    }
}
