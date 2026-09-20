//! Shared lifecycle for live HTTP integration servers.

use std::time::Duration;

use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_util::sync::CancellationToken;

pub type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub struct HttpServer {
    pub endpoint: String,
    cancellation: CancellationToken,
    task: JoinHandle<std::io::Result<()>>,
}

impl HttpServer {
    pub async fn start(config: StreamableHttpServerConfig) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind HTTP");
        let endpoint = format!(
            "http://{}/mcp",
            listener.local_addr().expect("listen address")
        );
        Self {
            endpoint,
            cancellation: config.cancellation_token.clone(),
            task: tokio::spawn(qimen_mcp::http::serve(listener, config)),
        }
    }

    pub async fn stop(mut self) -> TestResult {
        self.cancellation.cancel();
        tokio::time::timeout(Duration::from_secs(5), &mut self.task).await???;
        Ok(())
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        self.cancellation.cancel();
    }
}
