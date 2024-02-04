use futures::future::join_all;
use std::{io, time::Duration};
use tracing::{debug, Level};

use dummy_upstream::Server;
use l3::{
    config::{Config, Service, Upstream},
    daemon::Daemon,
};

use crate::dummy_downstream::Client;

mod dummy_downstream;
mod dummy_upstream;

const LB_PORT: u16 = 8000;

#[tokio::test(flavor = "multi_thread")]
async fn test_the_world() -> io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    debug!("testing debug");

    let upstream_ports = start_the_upstream().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    start_the_lb(&upstream_ports).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    run_downstream().await?;

    Ok(())
}

async fn start_the_upstream() -> io::Result<[u16; 4]> {
    let mut s1 = Server::listen().await?;
    let s1_port = s1.port;

    tokio::spawn(async move {
        s1.serve().await.expect("serve failure");
    });

    let mut s2 = Server::listen().await?;
    let s2_port = s2.port;

    tokio::spawn(async move {
        s2.serve().await.expect("serve failure");
    });

    let mut s3 = Server::listen().await?;
    let s3_port = s3.port;

    tokio::spawn(async move {
        s3.serve().await.expect("serve failure");
    });

    let mut s4 = Server::listen().await?;
    let s4_port = s4.port;

    tokio::spawn(async move {
        s4.serve().await.expect("serve failure");
    });

    Ok([s1_port, s2_port, s3_port, s4_port])
}

async fn start_the_lb(upstream_ports: &[u16]) -> io::Result<()> {
    let hosts: Vec<String> = upstream_ports
        .iter()
        .map(|p| format!("localhost:{}", p))
        .collect();

    let conf = Config {
        service: Service {
            host: String::from("localhost"),
            port: LB_PORT,
            max_msg_len: 100,
        },
        upstream: Upstream {
            hosts,
            connections: 25,
        },
    };

    let c = Box::leak(Box::new(conf));
    let daemon = Daemon::new(c);

    tokio::spawn(async move {
        daemon.run().await.expect("daemon run failure");
    });

    Ok(())
}

async fn run_downstream() -> io::Result<()> {
    const N_CLIENTS: usize = 5;
    const N_REQ: usize = 50;

    let mut handlers = vec![];
    for i in 0..N_CLIENTS {
        let handler = tokio::spawn(async move {
            let mut c = Client::connect(i, format!("localhost:{}", LB_PORT))
                .await
                .expect("should be able to connect to the load balancer");
            for j in 0..N_REQ {
                c.send_request(j, j % N_REQ == 0)
                    .await
                    .expect("send_request should not return an error");
            }
        });

        handlers.push(handler);
    }

    for h in join_all(handlers).await {
        if let Err(e) = h {
            panic!("{}", e);
        }
    }

    Ok(())
}
