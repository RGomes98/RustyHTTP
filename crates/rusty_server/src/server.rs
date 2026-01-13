use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Arc;

use super::RequestHandler;
use rusty_router::Router;
use rusty_utils::{ThreadPool, init_logger};
use tracing::{debug, error, info, warn};

pub struct ServerConfig {
    pub port: u16,
    pub host: Ipv4Addr,
    pub pool_size: usize,
}

pub struct Server {
    config: ServerConfig,
    router: Arc<Router>,
    listener: TcpListener,
}

impl Server {
    pub fn new(router: Router, config: ServerConfig) -> Result<Self, Error> {
        let address: SocketAddr = SocketAddr::from((config.host, config.port));
        debug!("Binding TCP listener to {address}");

        let listener: TcpListener = TcpListener::bind(address)?;
        let router: Arc<Router> = Arc::new(router);

        Ok(Self {
            config,
            router,
            listener,
        })
    }

    pub fn address(&self) -> String {
        self.listener
            .local_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into())
    }

    pub fn with_default_logger(self) -> Self {
        match init_logger() {
            Ok(_) => info!("Default logger initialized successfully"),
            Err(_) => warn!("Logger already initialized, using existing global subscriber"),
        };

        self
    }

    pub fn listen(&self) {
        info!("Server running on http://{}", self.address());
        let pool: ThreadPool = ThreadPool::new(self.config.pool_size);

        for stream in self.listener.incoming() {
            match stream {
                Err(e) => {
                    warn!("Connection attempt failed: {e}");
                }
                Ok(stream) => {
                    let peer_addr: Option<SocketAddr> = stream.peer_addr().ok();
                    let router: Arc<Router> = self.router.clone();
                    debug!("Accepted connection from {peer_addr:?}");

                    if let Err(e) = stream.set_nodelay(true) {
                        warn!("Failed to set 'TCP_NODELAY': {e}");
                    }

                    pool.schedule(move || {
                        let mut handler: RequestHandler = RequestHandler { router, stream };
                        if let Err(e) = handler.handle() {
                            error!("Failed to handle request from {peer_addr:?}: {e}");
                        }
                    });
                }
            }
        }
    }
}
