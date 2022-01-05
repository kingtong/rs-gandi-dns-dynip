
use anyhow::{Result, ensure, Context};
use isahc::{ReadResponseExt, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Str;
use serde_json::json;

const GANDI_BASE_URL: &str = "https://api.gandi.net/v5/livedns";
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
    fn new(base_url: &str, token: &str) -> GandiClient {
        GandiClient { base_url: base_url.to_string(), token: token.to_string() }
    }

    fn get_record_ip(&self, domain: &str, record: &str) -> Result<Option<String>> {
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

    fn upsert_record(&self, domain: &str, record: &str, ip: &str) -> Result<()> {
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

fn get_public_ip() -> Result<String> {
    let mut response = isahc::get("https://www.icanhazip.com/")?;
    Ok(response.text()?.trim_matches('\n').to_string())
}

fn main() -> Result<()>{
    let public_ip = get_public_ip().context("Failed to get public IP")?;

    println!("Public IP found: {}", public_ip);

    let gandi = GandiClient::new(GANDI_BASE_URL, "");

    let record_ip = gandi.get_record_ip("kingtong.org", "home")
        .context("Failed getting record IP")?;

    if let Some(ip) = record_ip {
        if ip == public_ip {
            println!("IPs are matching");
        } else {
            println!("Updating record IP from {} to {}", ip, public_ip);
            gandi.upsert_record("kingtong.org", "home", &public_ip);
        }
    } else {
        println!("Setting new record with IP {}", public_ip);
        gandi.upsert_record("kingtong.org", "home", &public_ip);
    }

    Ok(())
}
