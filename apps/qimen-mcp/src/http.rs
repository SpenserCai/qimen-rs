//! Streamable HTTP hosting with protocol and session handling provided by rmcp.

use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use tokio::net::TcpListener;

use crate::QimenServer;

/// Serve the Qimen tools at `/mcp` until the configuration's cancellation token
/// is cancelled. Cancellation closes active SDK sessions and drains HTTP work.
///
/// Origin validation is always enabled: requests carrying an Origin must match
/// `config.allowed_origins`, while non-browser clients may omit that header.
/// The SDK's default Host allowlist accepts loopback hosts only. Deployments
/// behind a reverse proxy must explicitly configure the trusted Host values;
/// the proxy is responsible for TLS and access authentication.
/// In the SDK's Origin allowlist, an omitted port matches all ports for that
/// scheme and host; an explicit port must also be present in the request Origin.
///
/// The preferred protocol is stateless. The SDK retains session handling for
/// legacy protocol revisions and supports both JSON and SSE responses.
pub async fn serve(
    listener: TcpListener,
    config: StreamableHttpServerConfig,
) -> std::io::Result<()> {
    let cancellation = config.cancellation_token.clone();
    let service: StreamableHttpService<QimenServer, LocalSessionManager> =
        StreamableHttpService::new(
            || Ok(QimenServer),
            Default::default(),
            config.enforce_origin_validation(),
        );
    let router = Router::new().route_service("/mcp", service);
    axum::serve(listener, router)
        .with_graceful_shutdown(cancellation.cancelled_owned())
        .await
}
