use pcap::{self, Capture, Device};
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Deserialize)]
struct Config {
    device: Option<String>,
    dash_macs: Vec<String>,
}

impl Config {
    fn new(pb: PathBuf) -> Self {
        let contents = fs::read_to_string(pb).expect("Unable to read config file");
        toml::from_str(&contents).expect("Invalid config")
    }
}

fn make_filter(macs: Vec<String>) -> String {
    format!(
        "arp and ({})",
        macs.into_iter()
            .map(|m| format!("(ether src host {})", m))
            .collect::<Vec<_>>()
            .join(" or ")
    )
}

fn listen(config: Config) {
    let mut cap = match config.device {
        Some(dev) => Capture::from_device(dev.as_str())
            .unwrap_or_else(|_| panic!("Could not find device {}", dev))
            .open()
            .unwrap_or_else(|_| panic!("Unable to open device {}", dev)),
        None => Capture::from_device(Device::lookup().expect("Unable to look up device"))
            .unwrap_or_else(|_| panic!("Unable to lookup device"))
            .open()
            .unwrap_or_else(|_| panic!("Unable to open device")),
    };
    cap.filter(make_filter(config.dash_macs).as_str())
        .expect("Unable to set filter");
    while let Ok(packet) = cap.next() {
        println!("got packet! {:?}", packet);
    }
}

fn main() {
    let config_path = match env::args().nth(1) {
        Some(path) => path,
        None => "config.toml".to_owned(),
    };
    let pb = PathBuf::from(config_path);
    let config = Config::new(pb);
    listen(config)
}
