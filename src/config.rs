use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;


#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub urls: HashMap<String, String>,
}

impl Config {
    pub fn merge(&mut self, other: Config) {
        for (key, value) in other.urls {
            self.urls.insert(key, value);
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = Config::default();

    // 1. Load from Home Directory
    if let Some(home_dir) = dirs::home_dir() {
        let home_config_path = home_dir.join(".fzopnr.toml");
        if home_config_path.exists() {
            let content = fs::read_to_string(&home_config_path)?;
            let home_config: Config = toml::from_str(&content)?;
            config.merge(home_config);
        }
    }

    // 2. Load from Root down to Current Directory (Recursive)
    // Actually, "parent directory recursively" usually implies starting from CWD and going up.
    // However, for configuration, we usually want specific (CWD) to override general (Parent).
    // So we should load from Root -> ... -> Parent -> CWD.
    // OR: Load CWD, then Parent, then Grandparent, but ONLY insert if not present?
    // The requirement says: "read configuration files from current working directory (and parent directory recurisvely)"
    // If I have a config in /project/.fzopnr.toml and /project/subdir/.fzopnr.toml
    // When in /project/subdir, I expect /project/subdir settings to override /project.
    // So I should load from top-down relative to the hierarchy found?
    // Easiest is to collect all paths from CWD up to root.
    // [CWD, Parent, Grandparent, ... Root]
    // Then iterate in reverse (Root -> Grandparent -> Parent -> CWD) and merge.

    let current_dir = env::current_dir()?;
    let mut paths_to_check = Vec::new();
    let mut dir = current_dir.as_path();

    loop {
        let config_path = dir.join(".fzopnr.toml");
        if config_path.exists() {
            paths_to_check.push(config_path);
        }
        if let Some(parent) = dir.parent() {
            dir = parent;
        } else {
            break;
        }
    }

    // Reverse to apply from most general (root) to most specific (cwd)
    paths_to_check.reverse();

    for path in paths_to_check {
        // Skip home config if we already loaded it and it appeared in the path traversal (unlikely if strictly checking .fzopnr.toml, but possible if CWD is Home)
        // Actually, if CWD is Home, we might load it twice.
        // Let's just load it. The merge will overwrite same keys with same values, effectively a no-op.
        // Or specific override specific.
        // If we want Home Config to be the *base*, and then project configs to override it.
        // The previous step loaded Home.
        // If Home is part of the path (e.g. I am in ~/projects), we don't want to load ~/.fzopnr.toml again if it was already loaded?
        // Or maybe we treat Home as a "Global Default" that is ALWAYS loaded first.
        // Then we load path configs.
        // If `path` == `home_config_path`, we might re-load.
        // Let's avoid checking uniqueness for now, it's cheap to re-parse.

        let content = fs::read_to_string(&path)?;
        let loaded_config: Config = toml::from_str(&content)?;
        config.merge(loaded_config);
    }

    Ok(config)
}
