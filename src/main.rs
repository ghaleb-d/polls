mod cli;
mod db;
mod models;
mod polls;
mod user;
mod vote;

use cli::run_cli;
use colored::*;
use db::init_pool;
use user::choose_user_flow;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Step 1: Initialize the DB connection pool
    let pool = init_pool().await?;
    sqlx::migrate!().run(&pool).await?;

    // Step 2: Create or fetch user
    let mut user = match choose_user_flow(&pool).await {
        Ok(u) => u,
        Err(e) => {
            eprintln!("❌ Could not create or fetch user: {}", e);
            return Ok(()); // Graceful exit
        }
    };
    println!("👤 Logged in user: {:#?}", user);
    println!("{} {}", "Welcome,".green(), user.username.blue().bold());

    run_cli(&pool, &mut user).await?;

    Ok(())
}
