use sqlx::{Sqlite, SqlitePool};
use crate::database::{DatabaseGeneralError};
use crate::models::database::channel::Channel;

pub async fn is_user_in_chat(database: &SqlitePool, user_id: i64, chat_id: i64) -> Result<bool, String> {
    let row = sqlx::query("SELECT 1 FROM chat_users WHERE chat_id = ? AND user_id = ?;")
        .bind(chat_id)
        .bind(user_id)
        .fetch_optional(database)
        .await
        .map_err(|e| {
            eprintln!("Failed to check if user is in chat: {}", e);
            "Failed to check if user is in chat".to_string()
        })?;

    Ok(row.is_some())
}

async fn is_chat_exist(database: &SqlitePool, chat_id: i64) -> Result<bool, String> {
    let row = sqlx::query("SELECT 1 FROM channels WHERE id = ?")
        .bind(chat_id)
        .fetch_optional(database)
        .await
        .map_err(|e| {
            eprintln!("Failed to check if chat exists: {}", e);
            "Failed to check if chat exists".to_string()
        })?;

    Ok(row.is_some())
}

pub async fn create_chat(database: &SqlitePool, name: &str) -> Result<Channel, DatabaseGeneralError> {

    let trimmed_name = name.trim();
    if trimmed_name.is_empty() {
        return Err(DatabaseGeneralError::InvalidInput);
    }

    let chat_info = sqlx::query_as::<Sqlite, Channel>("INSERT INTO channels (name) VALUES (?) RETURNING *;")
        .bind(trimmed_name)
        .fetch_one(database)
        .await
        .map_err(|_| DatabaseGeneralError::InternalError("Failed to create chat".to_string()))?;


    Ok(chat_info)
}

pub async fn remove_chat(database: &SqlitePool, chat_id: i64, user_id: i64) -> Result<(), DatabaseGeneralError> {

    let is_user_in_chat = is_user_in_chat(database, user_id, chat_id).await
        .map_err(|e| DatabaseGeneralError::InternalError(e))?;

    if is_user_in_chat == false { return Err(DatabaseGeneralError::NotFound) }

    let mut transaction = database.begin().await.map_err(|e| {
        eprintln!("Cannot start database transaction: {}", e);
        DatabaseGeneralError::InternalError("Failed to delete chat".to_string())
    })?;

    if let Err(e) = sqlx::query("DELETE FROM messages WHERE chat_id = ?;")
        .bind(chat_id)
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Failed to delete messages in chat: {}", e);

        if let Err(e) = transaction.rollback().await {
            eprintln!("Cannot rollback transaction: {}", e);
        }

        return Err(DatabaseGeneralError::InternalError("Failed to delete chat".to_string()))
    };


    if let Err(e) = sqlx::query("DELETE FROM chat_users WHERE chat_id = ?;")
        .bind(chat_id)
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Failed to delete users from the chat: {}", e);

        if let Err(e) = transaction.rollback().await {
            eprintln!("Cannot rollback transaction: {}", e);
        }

        return Err(DatabaseGeneralError::InternalError("Failed to delete chat".to_string()))
    };

    if let Err(e) = sqlx::query("DELETE FROM channels WHERE id = ?;")
        .bind(chat_id)
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Failed to delete the chat: {}", e);

        if let Err(e) = transaction.rollback().await {
            eprintln!("Cannot rollback transaction: {}", e);
        }

        return Err(DatabaseGeneralError::InternalError("Failed to delete chat".to_string()))
    }


    transaction.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        DatabaseGeneralError::InternalError("Failed to delete chat".to_string())
    })?;

    Ok(())
}

pub async fn add_user_to_chat(database: &SqlitePool, chat_id: i64, user_id: i64, key: &str, signature: &str) -> Result<(), DatabaseGeneralError> {
    if key.trim().is_empty() || signature.trim().is_empty() {
        return Err(DatabaseGeneralError::InvalidInput);
    }

    let is_chat_exist = is_chat_exist(database, chat_id).await
        .map_err(|e| DatabaseGeneralError::InternalError(e))?;

    if is_chat_exist == true { return Err(DatabaseGeneralError::NotFound) }

    let is_user_in_chat = is_user_in_chat(database, user_id, chat_id).await
        .map_err(|e| DatabaseGeneralError::InternalError(e))?;

    if is_user_in_chat == true { return Err(DatabaseGeneralError::AlreadyExists) }


    sqlx::query("INSERT INTO chat_users (chat_id, user_id, key, signature) VALUES (?, ?, ?, ?);")
        .bind(chat_id)
        .bind(user_id)
        .bind(key)
        .bind(signature)
        .execute(database)
        .await
        .map_err(|e| {
            eprintln!("Failed to add user {} to chat {}: {}", user_id, chat_id, e);
            DatabaseGeneralError::InternalError("Failed to add user to chat".to_string())
        })?;

    Ok(())
}

pub async fn get_channels_by_user_id(database: &SqlitePool, user_id: i64) -> Result<Vec<Channel>, DatabaseGeneralError> {
    let rows = sqlx::query_as::<Sqlite, Channel>("
            SELECT c.id, c.name, cu.key, cu.signature
            FROM channels c
            INNER JOIN chat_users cu ON c.id = cu.chat_id
            WHERE cu.user_id = ?;
        ")
        .bind(user_id)
        .fetch_all(database)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch chats for user {}: {}", user_id, e);
            DatabaseGeneralError::InternalError("Failed to fetch chats".to_string())
        })?;

    Ok(rows)
}

pub async fn remove_user_from_chat(database: &SqlitePool, chat_id: i64, user_id: i64) -> Result<(), DatabaseGeneralError> {
    let is_user_in_chat = is_user_in_chat(database, user_id, chat_id).await
        .map_err(|e| DatabaseGeneralError::InternalError(e))?;

    if is_user_in_chat == false { return Err(DatabaseGeneralError::NotFound) }

    let mut transaction = database.begin().await.map_err(|e| {
        eprintln!("Cannot start database transaction: {}", e);
        DatabaseGeneralError::InternalError("Failed to delete chat".to_string())
    })?;

    if let Err(e) = sqlx::query("DELETE FROM messages WHERE chat_id = ? AND user_id = ?;")
        .bind(chat_id)
        .bind(user_id)
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Failed to delete messages from the chat: {}", e);

        if let Err(e) = transaction.rollback().await {
            eprintln!("Cannot rollback transaction: {}", e);
        }

        return Err(DatabaseGeneralError::InternalError("Failed to remove user from chat".to_string()))
    };

    if let Err(e) = sqlx::query("DELETE FROM chat_users WHERE chat_id = ? AND user_id = ?;")
        .bind(chat_id)
        .bind(user_id)
        .execute(&mut *transaction)
        .await
    {
        eprintln!("Failed to delete user from the chat: {}", e);

        if let Err(e) = transaction.rollback().await {
            eprintln!("Cannot rollback transaction: {}", e);
        }

        return Err(DatabaseGeneralError::InternalError("Failed to remove user from chat".to_string()))
    };

    transaction.commit().await.map_err(|e| {
        eprintln!("Cannot commit transaction: {}", e);
        DatabaseGeneralError::InternalError("Failed to remove user from chat".to_string())
    })?;

    Ok(())
}