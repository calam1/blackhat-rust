use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::net::TcpStream;

pub async fn scan_ports(concurrency: usize, subdomain: Subdomain) -> Subdomain {
    let mut ret = subdomain.clone();

    // concurrent stream method, using channels
    let (input_tx, input_rx) = mpsc::channel(concurrency);
    let (output_tx, output_rx) = mpsc::channel(concurrency);

    tokio::spawn(async move {
        for port in MOST_COMMON_PORTS_100.into_iter() {
            input_tx.send(*port).await.unwrap();
        }
    });

    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
    input_rx_stream
        .for_each_concurrent(concurrency, |port| {
            let subdomain = subdomain.clone();
            let output_tx = output_tx.clone();
            async move {
                let port = scan_port(&subdomain.domain, port).await;
                if port.is_open {
                    let _ = output_tx.send(port).await;
                }
            }
        })
        .await;

    // close channel
    drop(output_tx);

    let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);
    ret.open_ports = output_rx_stream.collect().await;

    ret
}

async fn scan_port(hostname: &str, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return Port {
            port,
            is_open: false,
        };
    }

    let is_open = 
        match tokio::time::timeout(timeout,
            TcpStream::connect(&socket_addresses[0])).await {
            Ok(Ok(_)) => true,
            _ => false,
        };

    Port {
        port,
        is_open,
    }
}