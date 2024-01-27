use std::net::IpAddr;

pub struct NetApiClient {
    http_client: reqwest::blocking::Client
}

impl NetApiClient {
    pub fn new() -> NetApiClient {
        NetApiClient {
            http_client: reqwest::blocking::Client::builder()
                .user_agent("curl/7.81.0")
                .local_address(IpAddr::from([0, 0, 0, 0]))
                .build()
                .unwrap(),
        }
    }

    pub fn get_ip(&self) -> Result<String, reqwest::Error> {
        let resp: String = self.http_client
            .get("https://ip.hetzner.com")
            .send()?
            .error_for_status()?
            .text()?;

        Ok(resp.trim().to_string())
    }

}
