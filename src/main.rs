use serde_json;
use serde::Deserialize;
use std::{env, fs, io::Read, thread};
// use reqwest;
use tokio::{signal, time::sleep, time::Duration};


#[derive(Deserialize, Debug)]
struct ConfigFile{
    name: String,
    hostname: String,
    polling_interval: u8,
}

impl ConfigFile {
    fn parse_config_file(&config_path: &String) -> Vec<ConfigFile> {
        let mut config_file = fs::File::open(&config_path).unwrap();
        let mut buf = String::new();
        
        config_file.read_to_string(&mut buf);
        let config: Vec<ConfigFile> = serde_json::from_str(&buf).unwrap();
        return config;
    }
}

async fn check_website(hostname: &String) -> u8 {
    let result = reqwest::get(hostname).await.unwrap();

    return result.status().as_u16() as u8;
}

async fn watch_website(hostname: &String, interval: Duration) {
     loop {
        sleep(interval).await;
        check_website(hostname).await;
    }
}

#[tokio::main]
async fn main() {
    // use clap
    let args: Vec<String> = env::args().collect();

    let config = ConfigFile::parse_config_file(&args[1]);

    for site in config {
        // site.url, site.interval
        thread::spawn(watch_website(&site.hostname, Duration:new(site.polling_interval, 0)));
    }

    signal::ctrl_c().await;
}