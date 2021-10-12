use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use log::{warn, trace};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub exports: Vec<ExportConfig>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ExportConfig {
    pub name:           String,
    pub description:    String,
    pub r#type:         PrometheusType,
    pub command:        String
}

#[derive(Deserialize, Serialize, Clone)]
pub enum PrometheusType {
    Counter,
    Gauge,
}

pub const DEFAULT_PATH: &str = "/etc/prometheus-command-exporter/";
pub const DEFAULT_CONFIG_FILE: &str = "config.yml";

impl Default for Config {
    fn default() -> Self {
        Self {
            exports: vec![ExportConfig::default()]
        }
    }
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            name: String::from("random_number"),
            description: String::from("A random number"),
            r#type: PrometheusType::Gauge,
            command: String::from(r#"sh -c "echo $RANDOM" 2>&1"#)
        }
    }
}

impl Config {
    pub fn read() -> Result<Self> {
        let dir_path = Path::new(DEFAULT_PATH);
        trace!("Checking if configuration directory exists");
        if !dir_path.exists() {
            warn!("Config directory {} does not exist yet. Creating it now", DEFAULT_PATH);
            fs::create_dir(dir_path)?;
        }

        let config_file = dir_path.join(DEFAULT_CONFIG_FILE);
        trace!("Checking if configuration file exists");
        if !config_file.exists() {
            warn!("Config file {}{} does not exist yet. Creating default", DEFAULT_PATH, DEFAULT_CONFIG_FILE);
            let file = fs::File::create(&config_file)?;
            trace!("Writing default configuration file");
            serde_yaml::to_writer(file, &Self::default())?;

            warn!("Defualt configuration has been written. Please configure the program.");
            std::process::exit(0);
        }

        Self::read_from_path(&config_file)
    }

    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        trace!("Reading configuration file from {:?}", path.as_ref());
        let f = fs::File::open(path.as_ref())?;
        trace!("Deserializing read configuration");
        Ok(serde_yaml::from_reader(&f)?)
    }
}