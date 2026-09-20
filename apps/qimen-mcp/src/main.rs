//! Command-line entry point for the Qimen MCP server.

mod args;

use clap::Parser;
use qimen_mcp::QimenServer;
use rmcp::{ServiceExt, transport::stdio};
use tokio::net::TcpListener;

use args::{Options, Transport};

type AppResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> AppResult {
    let options = Options::parse();
    options.validate().unwrap_or_else(|error| error.exit());
    match options.transport {
        Transport::Stdio => {
            QimenServer.serve(stdio()).await?.waiting().await?;
        }
        Transport::StreamableHttp => serve_http(&options).await?,
    }
    Ok(())
}

async fn serve_http(options: &Options) -> AppResult {
    let listener = TcpListener::bind(options.bind_address()).await?;
    let address = listener.local_addr()?;
    let config = options.http_config(address);
    let cancellation = config.cancellation_token.clone();
    let server = qimen_mcp::http::serve(listener, config);
    tokio::pin!(server);
    eprintln!("Qimen MCP listening at http://{address}/mcp");
    tokio::select! {
        result = &mut server => result?,
        signal = shutdown_signal() => {
            cancellation.cancel();
            server.await?;
            signal?;
        }
    }
    Ok(())
}

async fn shutdown_signal() -> std::io::Result<()> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result,
            _ = terminate.recv() => Ok(()),
        }
    }
    #[cfg(not(unix))]
    tokio::signal::ctrl_c().await
}
