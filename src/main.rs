use std::env;

use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};

mod models;
mod services;
use services::{get_users, signup};

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    supertokens_url: String,
    supertokens_api_keys: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url: String = env::var("DATABASE_URL").unwrap_or(String::from(
        "postgresql://postgres:postgres@localhost:5432/postgres?schema=public",
    ));
    let supertokens_url: String =
        env::var("SUPERTOKENS_URL").unwrap_or(String::from("http://localhost:3567"));
    let supertokens_api_keys: String = env::var("SUPERTOKENS_API_KEYS").unwrap_or(String::from("Dh5RBOvnoqzGXcVEZe8OeyXcyR91JrYg0IRNQJ9mKqKC7nrCdynQ2FxNiGY8eQa5nfmPpHI3H2ZHwmZhRv2pi5YtbLtaoJtIvnl1Nmc0YnCpz8BVRnQZMcZCeCm0Wt1w"));

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Success in attempt to create and connect to database pool");
            pool
        }
        Err(err) => {
            println!(
                "Error in attempt to create and connect to database pool: {}",
                err
            );
            panic!()
        }
    };

    let app_state = AppState {
        db: pool,
        supertokens_url,
        supertokens_api_keys,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .service(get_users)
            .service(signup)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
