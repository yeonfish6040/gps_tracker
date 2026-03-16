mod migrator;

use dotenvy::dotenv;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    cli::run_cli(migrator::Migrator).await;
}
