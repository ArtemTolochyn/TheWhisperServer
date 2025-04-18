mod database;
mod api;
pub mod models;
mod services;
mod utils;

use actix_web::{App, HttpServer, web, middleware};
use crate::database::Database;
use std::sync::{Arc, Mutex};
use crate::api::{create_channel, get_channel_list, get_messages, join_channel, leave_channel, remove_channel, remove_message, send_message};
use crate::services::auth_service::auth_middleware;

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone)]
struct AppState
{
    database: Database,
    challenges: Arc<Mutex<std::collections::HashMap<String, String>>>,
    sessions: Arc<Mutex<std::collections::HashMap<String, i64>>>,
}



#[actix_web::main]
async fn main() -> Result<(), String> {

    let database = Database::initialize().await?;

    let app_state = AppState {
        database: database.clone(),
        challenges: Arc::new(Mutex::new(std::collections::HashMap::new())),
        sessions: Arc::new(Mutex::new(std::collections::HashMap::new()))
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(api::register_user)
            .service(api::login_request)
            .service(api::login_verify)
            .service(
                web::scope("api")
                    .wrap(middleware::from_fn(auth_middleware))
                    .service(create_channel)
                    .service(join_channel)
                    .service(leave_channel)
                    .service(remove_channel)
                    .service(get_channel_list)
                    .service(send_message)
                    .service(get_messages)
                    .service(remove_message)
            )
    })
        .bind(("127.0.0.1", 8080)).map_err(|_| "Cannot bind IP/Port")?
        .run()
        .await.map_err(|_| "Cannot run server")?;

    Ok(())
}