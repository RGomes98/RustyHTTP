use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use super::{Connection, ListenerError};
use forge_http::Response;
use forge_logging::init_logger;
use forge_router::Router;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn};

pub struct ListenerOptions {
    pub port: u16,
    pub host: Ipv4Addr,
}

pub struct Listener {
    router: Arc<Router>,
    options: ListenerOptions,
}

impl Listener {
    pub fn new(router: Router, options: ListenerOptions) -> Self {
        Self {
            options,
            router: Arc::new(router),
        }
    }

    pub fn with_default_logger(self) -> Self {
        match init_logger() {
            Ok(_) => info!("Default logger initialized successfully"),
            Err(_) => warn!("Logger already initialized, using existing global subscriber"),
        };

        self
    }

    pub async fn run(self) -> Result<(), Error> {
        let address: SocketAddr = SocketAddr::from((self.options.host, self.options.port));
        debug!("Binding TCP listener to {address}");

        let listener: TcpListener = TcpListener::bind(address).await?;
        info!("Listener running on http://{address}");

        loop {
            match listener.accept().await {
                Ok((stream, _address)) => {
                    let router: Arc<Router> = self.router.clone();

                    if let Err(e) = stream.set_nodelay(true) {
                        warn!("Failed to set 'TCP_NODELAY': {e}");
                    }

                    tokio::spawn(async move { Self::handle_connection(stream, router).await });
                }
                Err(e) => {
                    error!("Failed to accept connection: {e}");
                }
            }
        }
    }

    async fn handle_connection(stream: TcpStream, router: Arc<Router>) {
        let mut handler: Connection = Connection { router, stream };

        loop {
            if let Err(e) = handler.process_request().await {
                match e {
                    ListenerError::ConnectionClosed => break,
                    ListenerError::Http(e) => {
                        if (Response::new(e.status).send(&mut handler.stream).await).is_err() {
                            break;
                        }
                    }
                }
            }
        }
    }
}
