use std::{net::IpAddr, ops::RangeInclusive};

use clap::{Parser, Subcommand};

use rust_register_center::registry::ServiceInstance;

/// Service discovery CLI
#[derive(Parser)]
struct Cli {
    /// Service registry endpoint
    #[arg(short, long, required = true, default_value = "http://127.0.0.1:2312")]
    endpoint: String,

    /// Service name
    #[arg(short, long, required = true)]
    service: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query service instances
    Query,

    Register {
        /// Service host name, ip or domain name
        #[arg(short, long, required = true, default_value = "127.0.0.1", value_parser = validate_ip)]
        ipaddr: String,

        /// Service listen port
        #[arg(short, long, required = true, value_parser = validate_port)]
        port: u16,
    },
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn validate_port(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a port number", s))?;

    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "Port not in range {} - {}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn validate_ip(s: &str) -> Result<IpAddr, String> {
    s.parse().map_err(|e| format!("Invalid IP: {}", e))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Query => {
            let endpoint = &cli.endpoint;
            let service = &cli.service;

            let client = reqwest::Client::new();
            let response = client
                .get(format!("{}/registry/{}", endpoint, service))
                .send()
                .await
                .unwrap();

            let instances: Vec<ServiceInstance> = response.json().await.unwrap();

            println!("{:#?}", instances);
        }
        Commands::Register { ipaddr, port } => {
            let endpoint = &cli.endpoint;
            let service = &cli.service;

            println!(
                "Register service, endpoint: {}, {}, {}:{}",
                endpoint, service, ipaddr, port
            );

            // 发送注册请求逻辑
        }
    }
}
