mod dns_api_client;
mod net_api_client;

use std::env;
use std::error::Error;

pub struct Config {
    pub api_token: String,
    pub zone_name: String,
    pub record_name: String,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let api_token = match env::var("HETZNER_AUTH_API_TOKEN") {
            Ok(value) => value,
            Err(_) => return Err("HETZNER_AUTH_API_TOKEN env missing"),
        };

        let zone_name = match env::var("HETZNER_ZONE_NAME") {
            Ok(value) => value,
            Err(_) => return Err("HETZNER_ZONE_NAME env missing"),
        };

        let record_name = match env::var("HETZNER_RECORD_NAME") {
            Ok(value) => value,
            Err(_) => return Err("HETZNER_RECORD_NAME env missing"),
        };

        Ok(Config { api_token, zone_name, record_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>  {
    let dns_api_client = dns_api_client::DnsApiClient::new(&config.api_token);
    let net_api_client = net_api_client::NetApiClient::new();

    let zones = match dns_api_client.get_zones() {
        Ok(zones) => zones,
        Err(e) => Err(e)?,
    };

    let zone = zones.into_iter()
        .find(|zone| zone.name == config.zone_name)
        .ok_or("Zone not found")?;

    let records = match dns_api_client.get_records(&zone.id) {
        Ok(records) => records,
        Err(e) => Err(e)?,
    };

    let record = records.into_iter()
        .find(|record| record.name == config.record_name)
        .ok_or("Record not found")?;

    let current_ip = match net_api_client.get_ip() {
        Ok(ip) => ip,
        Err(e) => Err(e)?,
    };

    if record.value == current_ip {
        println!("IP for {}.{} is up-to-date.", record.name, zone.name);
    } else {
        print!("Updating IP for {}.{} to {}... ", record.name, zone.name, current_ip);
        match dns_api_client.update_record(&zone.id, &record.id, &record.name, &current_ip) {
            Ok(()) => {},
            Err(e) => Err(e)?,
        };
        println!("done.");
    }

    Ok(())
}