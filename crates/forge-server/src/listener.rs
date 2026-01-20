use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};
use std::num::NonZero;
use std::sync::Arc;

use super::{Connection, ListenerError};
use forge_http::Response;
use forge_logging::init_logger;
use forge_router::Router;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::{Builder, Runtime};
use tracing::{debug, error, info, warn};

pub struct ListenerOptions {
    pub port: u16,
    pub host: Ipv4Addr,
}

pub struct Listener {
    runtime: Runtime,
    router: Arc<Router>,
    address: SocketAddr,
    listener: TcpListener,
}

impl Listener {
    pub fn new(router: Router, config: ListenerOptions) -> Result<Self, Error> {
        let address: SocketAddr = SocketAddr::from((config.host, config.port));
        debug!("Binding TCP listener to {address}");

        let cpu: usize = std::thread::available_parallelism()
            .map(|n: NonZero<usize>| n.get())
            .unwrap_or(8);

        let runtime: Runtime = Builder::new_multi_thread()
            .worker_threads(cpu * 12)
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

    pub fn run(self) {
        info!("Listener running on http://{}", self.address);

        self.runtime.block_on(async {
            loop {
                match self.listener.accept().await {
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
        });
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
