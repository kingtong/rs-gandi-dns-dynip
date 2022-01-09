
use anyhow::{ensure, Result};
use isahc::{ReadResponseExt, Request, RequestExt};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub const GANDI_BASE_URL: &str = "https://api.gandi.net/v5/livedns";
const GANDI_RECORD_TTL: u32 = 300;

pub struct GandiClient {
    base_url: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
struct GandiLiveDNSRecord {
    rrset_href: String,
    rrset_name: String,
    rrset_type: String,
    rrset_values: Vec<String>,
    rrset_ttl: u32,
}

impl GandiClient {
    pub fn new(base_url: &str, token: &str) -> GandiClient {
        GandiClient { base_url: base_url.to_string(), token: token.to_string() }
    }

    pub fn get_record_ip(&self, domain: &str, record: &str) -> Result<Option<String>> {
        let mut response = Request::get(format!("{}/domains/{}/records/{}", self.base_url, domain, record))
            .header("Authorization", format!("Apikey {}", self.token))
            .header("Content-Type", "application/json")
            .body(())?
            .send()?;

        ensure!(response.status().as_u16() == 200, "Unexpected status code");

        let mut records: Vec<GandiLiveDNSRecord> = response.json()?;

        if records.len() == 0 {
            println!("Record not declared");
            return Ok(None)
        }

        ensure!(records.len() == 1, "Non unique record received");

        let mut record = records.pop().unwrap();

        ensure!(record.rrset_type == "A", "Record is not an alias");
        ensure!(record.rrset_values.len() == 1, "Multiple values for record");

        Ok(record.rrset_values.pop())
    }

    pub fn upsert_record(&self, domain: &str, record: &str, ip: &str) -> Result<()> {
        let response = Request::put(format!("{}/domains/{}/records/{}", self.base_url, domain, record))
            .header("Authorization", format!("Apikey {}", self.token))
            .header("Content-Type", "application/json")
            .body(json!({
                "items": [
                    {
                        "rrset_type": "A",
                        "rrset_values": [ip],
                        "rrset_ttl": GANDI_RECORD_TTL,
                    }
                ]
            }).to_string())?
            .send()?;

        ensure!(response.status().as_u16() == 201, "Unexpected status code");

        Ok(())
    }
}
