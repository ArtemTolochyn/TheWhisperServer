use sqlx::{Sqlite, SqlitePool};
use crate::database::{ChatInfo, DatabaseGeneralError};

pub async fn create_chat(database: &SqlitePool, name: &str) -> Result<ChatInfo, DatabaseGeneralError> {

    let trimmed_name = name.trim();
    if trimmed_name.is_empty() {
        return Err(DatabaseGeneralError::InvalidInput);
    }

    let query = "INSERT INTO channels (name) VALUES (?) RETURNING id, name, last_edited;";

    let chat_info = sqlx::query_as::<Sqlite, ChatInfo>(query)
        .bind(trimmed_name)
        .fetch_one(database)
        .await
        .map_err(|_| DatabaseGeneralError::InternalError("Failed to create chat".to_string()))?;


    Ok(chat_info)
}

async fn is_user_in_chat(database: &SqlitePool, user_id: i64, chat_id: i64) -> Result<bool, String> {
    let query = "SELECT 1 FROM chat_users WHERE chat_id = ? AND user_id = ?;";

    let row = sqlx::query(query)
        .bind(chat_id)
        .bind(user_id)
        .fetch_optional(database)
        .await
        .map_err(|e| {
            eprintln!("");
            "Failed to check if user is in chat".to_string()
        })?;

    Ok(row.is_some())
}

pub async fn remove_chat(database: &SqlitePool, chat_id: i64, user_id: i64) -> Result<(), DatabaseGeneralError> {

    let is_user_in_chat = is_user_in_chat(database, user_id, chat_id).await
        .map_err(|e| DatabaseGeneralError::InternalError(e))?;

    if is_user_in_chat == false
    {
        return Err(DatabaseGeneralError::NotFound)
    }

    let delete_messages_query = "DELETE FROM messages WHERE chat_id = ?;";

    let delete_messages_result = sqlx::query(delete_messages_query)
        .bind(chat_id)
        .execute(database)
        .await;

    if let Err(e) = delete_messages_result {
        eprintln!("Failed to delete messages in chat: {}", e);
        return Err(DatabaseGeneralError::InternalError("Failed to delete messages in chat".to_string()));
    }

    let delete_chat_users_query = r#"
            DELETE FROM chat_users WHERE chat_id = ?;
        "#;

    let delete_chat_users_result = sqlx::query(delete_chat_users_query)
        .bind(chat_id)
        .execute(&self.database)
        .await;

    if let Err(e) = delete_chat_users_result {
        eprintln!("Failed to delete chat_users entries: {}", e);
        return DatabaseGeneralResult_legacy::InternalError("Failed to delete chat_users entries".to_string());
    }

    let delete_chat_query = r#"
            DELETE FROM channels WHERE id = ?;
        "#;

    let delete_chat_result = sqlx::query(delete_chat_query)
        .bind(chat_id)
        .execute(&self.database)
        .await;

    match delete_chat_result {
        Ok(_) => DatabaseGeneralResult_legacy::Ok(()),
        Err(e) => {
            eprintln!("Failed to delete chat: {}", e);
            DatabaseGeneralResult_legacy::InternalError("Failed to delete chat".to_string())
        }
    }
}