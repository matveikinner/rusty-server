use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};

mod models;
mod services;
use services::get_users;

const DATABASE_URL: &str = "postgresql://postgres:postgres@localhost:5432/postgres?schema=public";

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(get_users)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
