use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use crate::{RequestHandler, ServerError};
use rusty_http::Response;
use rusty_router::Router;
use rusty_utils::init_logger;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::{Builder, Runtime};
use tracing::{debug, error, info, warn};

pub struct ServerConfig {
    pub port: u16,
    pub host: Ipv4Addr,
    pub pool_size: usize,
}

pub struct Server {
    runtime: Runtime,
    router: Arc<Router>,
    address: SocketAddr,
    listener: TcpListener,
}

impl Server {
    pub fn new(router: Router, config: ServerConfig) -> Result<Self, Error> {
        let address: SocketAddr = SocketAddr::from((config.host, config.port));
        debug!("Binding TCP listener to {address}");

        let runtime: Runtime = Builder::new_multi_thread()
            .worker_threads(config.pool_size)
            .enable_all()
            .build()?;

        let listener: TcpListener = runtime.block_on(TcpListener::bind(address))?;
        let router: Arc<Router> = Arc::new(router);

        Ok(Self {
            router,
            runtime,
            address,
            listener,
        })
    }

    pub fn with_default_logger(self) -> Self {
        match init_logger() {
            Ok(_) => info!("Default logger initialized successfully"),
            Err(_) => warn!("Logger already initialized, using existing global subscriber"),
        };

        self
    }

    pub fn listen(self) {
        info!("Server running on http://{}", self.address);

        self.runtime.block_on(async {
            loop {
                match self.listener.accept().await {
                    Ok((stream, _address)) => {
                        let router: Arc<Router> = self.router.clone();

                        if let Err(e) = stream.set_nodelay(true) {
                            warn!("Failed to set 'TCP_NODELAY': {e}");
                        }

                        tokio::spawn(async move { Self::process_connection(stream, router).await });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {e}");
                    }
                }
            }
        });
    }

    async fn process_connection(stream: TcpStream, router: Arc<Router>) {
        let mut handler: RequestHandler = RequestHandler { router, stream };

        loop {
            if let Err(e) = handler.handle().await {
                match e {
                    ServerError::ConnectionClosed => break,
                    ServerError::Http(e) => {
                        if (Response::new(e.status).write_to_stream(&mut handler.stream).await).is_err() {
                            break;
                        }
                    }
                }
            }
        }
    }
}
