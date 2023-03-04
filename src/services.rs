use actix_web::{get, web, HttpResponse, Responder};

use crate::models::user::User;

use crate::AppState;

#[get("/users")]
pub async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
    let query = "SELECT * FROM users";

    let users: Result<Vec<User>, sqlx::Error> =
        sqlx::query_as(query).fetch_all(&app_state.db).await;

    match users {
        Ok(users) => {
            println!("Success in attempt to retrieve users from database");
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            println!("Error in attempt to retrieve users from database: {}", err);
            HttpResponse::InternalServerError().into()
        }
    }
}
