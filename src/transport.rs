#![deny(warnings)]
#![allow(dead_code)] // Types will be used as implementation progresses

// STDIN/STDOUT and WebSocket transport handlers

use crate::error::{Result, TransportError};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

/// STDIN/STDOUT transport for MCP
#[derive(Default)]
pub struct StdioTransportHandler;

impl StdioTransportHandler {
    /// Create a new STDIN/STDOUT transport handler
    #[allow(clippy::default_constructed_unit_structs)] // Default is appropriate here
    pub fn new() -> Self {
        Self::default()
    }

    /// Read a JSON-RPC message from stdin (newline-delimited)
    pub async fn read_message(&mut self) -> Result<String> {
        let mut stdin = BufReader::new(io::stdin());
        let mut line = String::new();
        stdin.read_line(&mut line).await
            .map_err(TransportError::Io)?;
        Ok(line.trim().to_string())
    }

    /// Write a JSON-RPC message to stdout (newline-delimited)
    pub async fn write_message(&mut self, message: &str) -> Result<()> {
        let mut stdout = io::stdout();
        stdout.write_all(message.as_bytes()).await
            .map_err(TransportError::Io)?;
        stdout.write_all(b"\n").await
            .map_err(TransportError::Io)?;
        stdout.flush().await
            .map_err(TransportError::Io)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdio_transport_handler_creation() {
        let handler = StdioTransportHandler::new();
        // Just verify it can be created
        let _ = handler;
    }
}
