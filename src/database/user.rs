use sqlx::{Sqlite, SqlitePool};
use crate::database::{DatabaseGeneralError, User};

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

    sqlx::query("INSERT INTO users (username, public_key) VALUES (?, ?)")
        .bind(username)
        .bind(public_key)
        .execute(database)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.message().contains("UNIQUE constraint failed") {
                    return DatabaseGeneralError::AlreadyExists;
                }
            }
            eprintln!("Cannot add user to database: {}", e);
            DatabaseGeneralError::InternalError("Cannot add user to database".to_string())
        })?;

    println!("User '{}' registered", username);
    Ok(())
}


async fn get_user_by_query<T>(database: &SqlitePool, query: &str, bind_value: T) -> Result<User, DatabaseGeneralError>
where T: for<'q> sqlx::Encode<'q, Sqlite> + sqlx::Type<Sqlite> + Send,
{
    let user_option = sqlx::query_as::<Sqlite, User>(query)
        .bind(bind_value)
        .fetch_optional(database)
        .await
        .map_err(|e| {
            eprintln!("Failed to get user: {}", e);
            DatabaseGeneralError::InternalError("Failed to get user".to_string())
        })?;

    let user = match user_option {
        Some(user) => user,
        None => return Err(DatabaseGeneralError::NotFound),
    };
    
    Ok(user)
}