use clap::{Parser, Subcommand};

use rust_register_center::registry::ServiceInstance;

/// Service discovery CLI
#[derive(Parser)]
struct Cli {
    /// Service registry endpoint
    #[arg(short, long, required = true)]
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
        #[arg(short, long, required = true)]
        ipaddr: String,

        /// Service listen port
        #[arg(short, long, required = true)]
        port: u16,
    },
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
