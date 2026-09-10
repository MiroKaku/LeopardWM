//! Canonical `leopardwm-cli` binary.

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    leopardwm_cli::run().await
}
