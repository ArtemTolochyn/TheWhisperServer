use sqlx::{Row, Sqlite, SqlitePool};
use crate::database::{DatabaseGeneralError, Message};
use crate::database::channel::is_user_in_chat;

pub async fn add_message(database: &SqlitePool, chat_id: i64, user_id: i64, content: &str) -> Result<i64, DatabaseGeneralError> {
    if content.trim().is_empty() {
        return Err(DatabaseGeneralError::InvalidInput)
    }

    let is_user_in_chat = is_user_in_chat(database, user_id, chat_id).await
        .map_err(|e| DatabaseGeneralError::InternalError(e))?;

    if is_user_in_chat == false { return Err(DatabaseGeneralError::NotFound) }

    let row = sqlx::query("
            INSERT INTO messages (chat_id, user_id, content)
            VALUES (?, ?, ?)
            RETURNING id;
        ")
        .bind(chat_id)
        .bind(user_id)
        .bind(content)
        .fetch_one(database)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert message: {}", e);
            DatabaseGeneralError::InternalError("Failed to insert message".to_string())
        })?;

    let message_id: i64 = row.try_get("id").map_err(|e| {
        eprintln!("Failed to add message to database: {}", e);
        DatabaseGeneralError::InternalError("Failed to add message to database".to_string())
    })?;

    Ok(message_id)
}

pub async fn remove_message(database: &SqlitePool, message_id: i64, user_id: i64) -> Result<(), DatabaseGeneralError> {
    let row_option = sqlx::query("SELECT chat_id FROM messages WHERE id = ? AND user_id = ?;")
        .bind(message_id)
        .bind(user_id)
        .fetch_optional(database)
        .await
        .map_err(|e| {
            eprintln!("Failed to check message existence: {}", e);
            DatabaseGeneralError::InternalError("Failed to check message existence".to_string())
        })?;

    if let None = row_option {
        return Err(DatabaseGeneralError::NotFound)
    }

    sqlx::query("DELETE FROM messages WHERE id = ?;")
        .bind(message_id)
        .execute(database)
        .await
        .map_err(|e| {
            eprintln!("Cannot delete message from the database: {}", e);
            DatabaseGeneralError::InternalError("Cannot remove message from the database".to_string())
        })?;

    Ok(())
}

pub async fn get_messages(database: &SqlitePool, chat_id: i64, user_id: i64, before_message_id: Option<i64>) -> Result<Vec<Message>, DatabaseGeneralError> {
    let limit = 50;

    let is_user_in_chat = is_user_in_chat(database, user_id, chat_id).await
        .map_err(|e| DatabaseGeneralError::InternalError(e))?;

    if is_user_in_chat == false { return Err(DatabaseGeneralError::NotFound) }

    match before_message_id {
        Some(before_id) => {
            sqlx::query_as::<Sqlite, Message>("
                 SELECT id, chat_id, user_id, content, timestamp
                FROM messages
                WHERE chat_id = ? AND id < ?
                ORDER BY id DESC
                LIMIT ?;
            ")
                .bind(chat_id)
                .bind(before_id)
                .bind(limit)
                .fetch_all(database)
                .await
                .map_err(|e| {
                    eprintln!("Cannot get messages: {}", e);
                    DatabaseGeneralError::InternalError("Cannot get messages".to_string())
                })
        }

        None => {
            sqlx::query_as::<Sqlite, Message>("
                 SELECT id, chat_id, user_id, content, timestamp
                FROM messages
                ORDER BY id DESC
                LIMIT ?;
            ")
                .bind(chat_id)
                .bind(limit)
                .fetch_all(database)
                .await
                .map_err(|e| {
                    eprintln!("Cannot get messages: {}", e);
                    DatabaseGeneralError::InternalError("Cannot get messages".to_string())
                })
        }
    }
}