use anyhow::Result;
use tracing::{info, warn};

use crate::cli::GlobalOpts;

pub async fn login_command(endpoint: Option<String>, global_opts: &GlobalOpts) -> Result<()> {
    info!("Starting login process");

    let endpoint = endpoint.unwrap_or_else(|| "https://api.max.dev".to_string());

    if global_opts.is_json_output() {
        println!(
            r#"{{"status": "not_implemented", "message": "Login functionality coming soon"}}"#
        );
    } else {
        println!("🔐 Login functionality coming soon...");
        println!("Will authenticate with endpoint: {}", endpoint);
    }

    warn!("Login not yet implemented - placeholder only");
    Ok(())
}
