use nekotimer_shared::TimerConfigFile;
use std::fs;

pub fn load_config(path: &str) -> Result<TimerConfigFile, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: TimerConfigFile = serde_json::from_str(&contents)?;
    Ok(config)
}

pub fn save_config(path: &str, config: &TimerConfigFile) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)?;
    Ok(())
}
