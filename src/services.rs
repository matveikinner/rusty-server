use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::models::auth::{Signup, SignupResponse};
use crate::models::user::User;

use crate::AppState;

#[get("/users")]
pub async fn get_users(app_state: Data<AppState>) -> impl Responder {
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

// https://app.swaggerhub.com/apis/supertokens/CDI/2.18.2#/EmailPassword%20Recipe/emailPasswordSignup
#[post("/signup")]
pub async fn signup(state: Data<AppState>, body: Json<Signup>) -> impl Responder {
    let client = Client::new();
    let mut header_map = HeaderMap::new();

    header_map.insert("rid", HeaderValue::from_str("emailpassword").unwrap());
    header_map.insert(
        "api-key",
        HeaderValue::from_str(&state.supertokens_api_keys).unwrap(),
    );
    header_map.insert("cdi-version", HeaderValue::from_str("2.18").unwrap());

    println!("{:#?}", body);

    let credentials = format!(
        r#"{{"email": "{}", "password": "{}"}}"#,
        body.email, body.password
    );

    println!("{}", credentials);

    let url = &format!(r#"{}{}"#, &state.supertokens_url, "/recipe/signup");

    match client
        .post(url)
        .headers(header_map)
        .body(credentials)
        .send()
        .await
    {
        Ok(response) => {
            println!("Success in attempt to signup {:?}", response);
            match response.json::<SignupResponse>().await {
                Ok(result) => {
                    println!("Success in attempt to deserialize SuperTokensResponse");
                    HttpResponse::Ok().json(result)
                }
                Err(err) => {
                    println!(
                        "Error in attempt to deserialize SuperTokensResponse {}",
                        err
                    );
                    HttpResponse::InternalServerError().json(format!(r#"{{"error": "{}"}}"#, err))
                }
            }
        }
        Err(err) => {
            println!("Error in attempt to signup: {}", err);
            HttpResponse::InternalServerError().into()
        }
    }
}
