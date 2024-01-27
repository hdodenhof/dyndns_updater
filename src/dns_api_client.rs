use reqwest::{header, Url};
use reqwest::header::HeaderValue;
use serde::Deserialize;
use serde_json::json;

pub struct DnsApiClient {
    http_client: reqwest::blocking::Client,
    base_url: Url,
}

impl DnsApiClient {
    pub fn new(token: &str) -> DnsApiClient {
        let mut headers = header::HeaderMap::new();
        headers.insert("Auth-API-Token", HeaderValue::from_str(&token).unwrap());

        DnsApiClient {
            http_client: reqwest::blocking::Client::builder().default_headers(headers).build().unwrap(),
            base_url: Url::parse("https://dns.hetzner.com/api/v1/").unwrap(),
        }
    }

    pub fn get_zones(&self) -> Result<Vec<Zone>, reqwest::Error> {
        let resp: GetZonesResponse = self.http_client
            .get(self.build_url("zones"))
            .send()?
            .error_for_status()?
            .json()?;

        Ok(resp.zones)
    }

    pub fn get_records(&self, zone_id: &str) -> Result<Vec<Record>, reqwest::Error> {
        let resp: GetRecordsResponse = self.http_client
            .get(self.build_url("records"))
            .query(&[("zone_id", zone_id)])
            .send()?
            .error_for_status()?
            .json()?;

        Ok(resp.records)
    }

    pub fn update_record(&self, zone_id: &str, id: &str, name: &str, value: &str) -> Result<(), reqwest::Error> {
        let req_body = json!({
            "zone_id": zone_id,
            "type": "A",
            "name": name,
            "value": value
        });

        self.http_client
            .put(self.build_url(&format!("records/{}", id)))
            .json(&req_body)
            .send()?
            .error_for_status()?;

        Ok(())
    }

    fn build_url(&self, path: &str) -> Url {
        self.base_url.join(path).unwrap()
    }
}

#[derive(Deserialize)]
struct GetZonesResponse {
    zones: Vec<Zone>,
}

#[derive(Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
struct GetRecordsResponse {
    records: Vec<Record>,
}

#[derive(Deserialize)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub value: String,
}
