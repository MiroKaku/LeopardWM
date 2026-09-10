//! `lwm` — short alias for `leopardwm-cli`. Same entry point, same behavior.

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    leopardwm_cli::run().await
}
