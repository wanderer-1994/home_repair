use error::{Error, Result};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

/// Register a [`TcpListener`] on a random port that is assigned by the OS.
pub async fn register_random_os_socket() -> Result<TcpListener> {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 0)))
        .await
        .map_err(|e| Error::internal(format!("Cannot register service port {e:?}")))?;
    Ok(listener)
}
