mod error;
pub use error::Error;

mod subdomains;
mod model;
use model::Subdomain;

mod ports;
mod common_ports;

use std::env;
use std::time::Duration;
use reqwest::{blocking::Client, redirect};
use anyhow::Result;

use std::vec::Vec;

fn main() -> Result<()> {
// fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::CliUsage.into());
    }

    let target = &args[1].as_str();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()?;

    // pool.install is required to use our custom threadpool, instad of rayon's default one
    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("{}", port.port)
            }
            println!("");
        }
    });

    Ok(())
}
