mod database;
mod api;
mod models;
mod services;
mod utils;

use std::collections::HashMap;
use actix_web::{App, HttpServer, web, middleware};
use crate::database::Database;
use std::sync::{Arc, Mutex};
use crate::models::UserSessionsData;
use crate::services::auth_service::auth_middleware;

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone)]
struct AppState
{
    database: Database,
    challenges: Arc<Mutex<HashMap<String, String>>>,
    sessions: Arc<Mutex<HashMap<String, UserSessionsData>>>
}



#[actix_web::main]
async fn main() -> Result<(), String> {

    let database = Database::initialize().await?;

    let app_state = AppState {
        database: database.clone(),
        challenges: Arc::new(Mutex::new(HashMap::new())),
        sessions: Arc::new(Mutex::new(HashMap::new()))
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(
                web::scope("api/auth")
                    .service(api::auth::register)
                    .service(
                        web::scope("login")
                            .service(api::auth::request)
                            .service(api::auth::validate)
                    )
            )
            .service(
                web::scope("api")
                    .wrap(middleware::from_fn(auth_middleware))
                    .service(
                        web::scope("user")
                            .service(api::user::channels)
                    )
                    .service(
                        web::scope("channel")
                            .service(api::channel::create_channel)
                            .service(api::channel::join_channel)
                            .service(api::channel::leave_channel)
                            .service(api::channel::remove_channel)
                    )
                    .service(
                        web::scope("message")
                            .service(api::message::send_message)
                            .service(api::message::get_messages)
                            .service(api::message::remove_message)
                    )
            )
    })
        .bind(("127.0.0.1", 8080)).map_err(|_| "Cannot bind IP/Port")?
        .run()
        .await.map_err(|_| "Cannot run server")?;

    Ok(())
}