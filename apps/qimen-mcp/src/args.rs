//! Command-line configuration for the available MCP transports.

use std::net::{Ipv4Addr, SocketAddr};

use axum::http::{Uri, uri::Authority};
use clap::{Parser, ValueEnum};
use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub(crate) enum Transport {
    #[default]
    Stdio,
    StreamableHttp,
}

#[derive(Parser)]
#[command(
    version,
    about = "奇门遁甲 MCP 服务 / Qimen Dunjia MCP server",
    long_about = "Run the read-only bazi and paipan tools over stdio (default) or Streamable HTTP.\n\
                  MCP 2026-07-28 discovery and legacy initialization are supported.\n\
                  Standard output is reserved for MCP JSON-RPC messages.\n\
                  HTTP serves /mcp without built-in TLS or authentication; protect remote deployments with a reverse proxy."
)]
pub(crate) struct Options {
    /// MCP transport.
    #[arg(long, value_enum, default_value_t)]
    pub(crate) transport: Transport,

    /// HTTP listen address [default: 127.0.0.1:8080].
    #[arg(long, value_name = "ADDRESS")]
    pub(crate) bind: Option<SocketAddr>,

    /// Additional trusted HTTP Host authority; repeat for multiple hosts.
    #[arg(long, value_name = "HOST[:PORT]", value_parser = parse_host)]
    allow_host: Vec<String>,

    /// Trusted HTTP Origin; repeat as needed. An omitted port allows any port.
    /// By default all Origin headers are rejected.
    #[arg(long, value_name = "ORIGIN", value_parser = parse_origin)]
    allow_origin: Vec<String>,
}

impl Options {
    pub(crate) fn validate(&self) -> Result<(), clap::Error> {
        if self.transport == Transport::Stdio
            && (self.bind.is_some() || !self.allow_host.is_empty() || !self.allow_origin.is_empty())
        {
            return Err(clap::Error::raw(
                clap::error::ErrorKind::ArgumentConflict,
                "--bind, --allow-host and --allow-origin require --transport streamable-http",
            ));
        }
        Ok(())
    }

    pub(crate) fn bind_address(&self) -> SocketAddr {
        self.bind
            .unwrap_or_else(|| SocketAddr::from((Ipv4Addr::LOCALHOST, 8080)))
    }

    pub(crate) fn http_config(&self, address: SocketAddr) -> StreamableHttpServerConfig {
        let mut config = StreamableHttpServerConfig::default().enforce_origin_validation();
        if !address.ip().is_unspecified() {
            config.allowed_hosts.push(address.to_string());
        }
        config.allowed_hosts.extend(self.allow_host.iter().cloned());
        config.allowed_origins.clone_from(&self.allow_origin);
        config
    }
}

fn parse_host(value: &str) -> Result<String, String> {
    let authority: Authority = value.parse().map_err(|_| "expected HOST or HOST:PORT")?;
    if authority.host().is_empty()
        || value.contains(['@', '*'])
        || (authority.as_str().len() > authority.host().len() && authority.port_u16().is_none())
    {
        return Err("expected a trusted host without user information or wildcards".into());
    }
    Ok(value.to_owned())
}

fn parse_origin(value: &str) -> Result<String, String> {
    let uri: Uri = value
        .parse()
        .map_err(|_| "expected an http:// or https:// origin")?;
    if !matches!(uri.scheme_str(), Some("http" | "https"))
        || uri.path() != "/"
        || uri.query().is_some()
        || value.contains('#')
    {
        return Err(
            "expected an http:// or https:// origin without a path, query or fragment".into(),
        );
    }
    let authority = uri.authority().ok_or("origin must contain a host")?;
    parse_host(authority.as_str())?;
    Ok(value.to_owned())
}
