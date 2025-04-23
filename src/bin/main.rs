use loco_rs::cli;
use migration::Migrator;
use stat_api::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
