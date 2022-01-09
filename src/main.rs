
use anyhow::{Context, Result};
use clap::Parser;
use isahc::prelude::*;

use gandi::{GandiClient, GANDI_BASE_URL};
use settings::Settings;

mod gandi;
mod settings;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(long)]
    config: Option<String>,
    #[clap(long, default_value_t = Default::default())]
    api_key: String,
    #[clap(long, default_value_t = Default::default())]
    domain: String,
    #[clap(long, default_value_t = Default::default())]
    record: String,
    #[clap(long)]
    ip: Option<String>,
}

fn get_public_ip() -> Result<String> {
    let mut response = isahc::get("https://www.icanhazip.com/")?;
    Ok(response.text()?.trim_matches('\n').to_string())
}

fn main() -> Result<()>{
    let args = Args::parse();

    let settings = if let Some(path) = args.config {
        Settings::load(&path)?
    } else {
        Settings {
            api_key: args.api_key,
            domain: args.domain,
            record: args.record,
            ip: args.ip,
        }
    };

    println!("Record: {}.{}", settings.record, settings.domain);

    let public_ip = settings.ip.unwrap_or(get_public_ip().context("Failed to get public IP")?);

    println!("Public IP: {}", public_ip);

    let gandi = GandiClient::new(GANDI_BASE_URL, &settings.api_key);

    let record_ip = gandi.get_record_ip(&settings.domain,  &settings.record)
        .context("Failed to get record IP")?;

    if let Some(ip) = record_ip {
        println!("Record IP: {}", ip);

        if ip == public_ip {
            println!("IPs are matching");
        } else {
            println!("Updating record IP from {} to {}", ip, public_ip);
            gandi.upsert_record(&settings.domain, &settings.record, &public_ip)?;
        }
    } else {
        println!("Setting new record with IP {}", public_ip);
        return gandi.upsert_record(&settings.domain, &settings.record, &public_ip);
    }

    Ok(())
}
