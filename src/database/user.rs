use sqlx::{Sqlite, SqlitePool};
use crate::database::{DatabaseGeneralError, DatabaseGeneralResult_legacy, User};

pub async fn get_user_by_username(database: &SqlitePool, username: &str) -> Result<User, DatabaseGeneralError> {
    get_user_by_query(
        database,
        "SELECT id, username, public_key FROM users WHERE username = ?;",
        username.to_string(),
    ).await
}

pub async fn get_user_by_id(database: &SqlitePool, user_id: i64) -> Result<User, DatabaseGeneralError> {
    get_user_by_query(
        database,
        "SELECT id, username, public_key FROM users WHERE id = ?;",
        &user_id,
    ).await
}

pub async fn register_user(database: &SqlitePool, username: &str, public_key: &str) -> Result<(), DatabaseGeneralError> {
    if username.trim().is_empty() {
        return Err(DatabaseGeneralError::InvalidInput);
    }

    let query = "INSERT INTO users (username, public_key) VALUES (?, ?)";

    let result = sqlx::query(query)
        .bind(username)
        .bind(public_key)
        .execute(database)
        .await;

    if let Err(error) = result {
        if let Some(db_err) = error.as_database_error() {
            if db_err.message().contains("UNIQUE constraint failed") {
                return Err(DatabaseGeneralError::AlreadyExists);
            }
        }

        eprintln!("Cannot add user to database: {}", error);
        return Err(DatabaseGeneralError::InternalError("Cannot add user to database".to_string()))
    }

    println!("User '{}' registered", username);
    Ok(())
}


async fn get_user_by_query<T>(database: &SqlitePool, query: &str, bind_value: T) -> Result<User, DatabaseGeneralError>
where T: for<'q> sqlx::Encode<'q, Sqlite> + sqlx::Type<Sqlite> + Send,
{
    let result = sqlx::query_as::<Sqlite, User>(query)
        .bind(bind_value)
        .fetch_optional(database)
        .await;

    let user_option = match result {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Failed to get user: {}", e);
            return Err(DatabaseGeneralError::InternalError("Failed to get user".to_string()));
        }
    };

    let user = match user_option {
        Some(user) => user,
        None => return Err(DatabaseGeneralError::NotFound),
    };



    Ok(user)
}