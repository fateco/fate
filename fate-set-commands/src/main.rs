use anyhow::{Result, anyhow};
use fate::get_commands;
use std::env;
use twilight_http::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(
        env::args()
            .nth(1)
            .ok_or_else(|| anyhow!("please pass token arg"))?,
    );
    client
        .interaction(client.current_user_application().await?.model().await?.id)
        .set_global_commands(&get_commands())
        .await?;
    Ok(())
}
