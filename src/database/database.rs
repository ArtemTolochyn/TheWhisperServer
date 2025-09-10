use sqlx::{FromRow, Row, Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use crate::DB_URL;
use serde::Serialize;
use crate::database::initialization::{create_channels_table, create_chat_users_table, create_messages_table, create_users_table};

pub enum DatabaseGeneralResult_legacy<T> {
    Ok(T),
    NotFound,
    AlreadyExists,
    InvalidInput,
    InternalError(String),
}

pub enum DatabaseGeneralError {
    NotFound,
    AlreadyExists,
    InvalidInput,
    InternalError(String),
}

#[derive(Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub public_key: String,
}

#[derive(Serialize, FromRow)]
pub struct Chat {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub signature: String,
}

#[derive(Serialize, FromRow)]
pub struct ChatInfo {
    pub id: i64,
    pub name: String,
}



#[derive(Debug, Serialize)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Clone)]
pub struct Database
{
    database: SqlitePool
}


impl Database
{
    pub async fn initialize() -> Result<Self, String> {
        if !Sqlite::database_exists(DB_URL).await.map_err(|_| "Cannot check if database exists")?
        {
            Sqlite::create_database(DB_URL).await.map_err(|_| "Cannot create database")?;
            println!("Database created");
        }

        let database = SqlitePool::connect(DB_URL).await.map_err(|_| "Cannot connect to database")?;
        println!("Database pool created");

        create_users_table(&database).await?;
        create_channels_table(&database).await?;
        create_chat_users_table(&database).await?;
        create_messages_table(&database).await?;

        let database_struct = Database { database };

        Ok(database_struct)
    }

    pub async fn create_chat(&self, name: &str) -> DatabaseGeneralResult_legacy<ChatInfo> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return DatabaseGeneralResult_legacy::InvalidInput;
        }

        let query = r#"
            INSERT INTO channels (name)
            VALUES (?)
            RETURNING id, name, last_edited;
        "#;

        let result = sqlx::query(query)
            .bind(trimmed_name)
            .fetch_one(&self.database)
            .await;

        match result {
            Ok(row) => {
                let id: i64 = match row.try_get("id") {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to extract id after chat creation: {}", e);
                        return DatabaseGeneralResult_legacy::InternalError("Failed to process chat creation result (id)".to_string());
                    }
                };
                let name_from_db: String = match row.try_get("name") {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to extract name after chat creation: {}", e);
                        return DatabaseGeneralResult_legacy::InternalError("Failed to process chat creation result (name)".to_string());
                    }
                };

                let chat_info = ChatInfo {
                    id,
                    name: name_from_db,
                };
                DatabaseGeneralResult_legacy::Ok(chat_info)
            },
            Err(e) => {
                eprintln!("Failed to create chat: {}", e);
                DatabaseGeneralResult_legacy::InternalError(format!("Failed to create chat: {}", e))
            }
        }
    }


    pub async fn remove_chat(&self, chat_id: i64, user_id: i64) -> DatabaseGeneralResult_legacy<()> {
        let check_user_query = r#"
            SELECT 1 FROM chat_users WHERE chat_id = ? AND user_id = ?;
        "#;

        let user_in_chat = sqlx::query(check_user_query)
            .bind(chat_id)
            .bind(user_id)
            .fetch_optional(&self.database)
            .await;

        if let Err(e) = user_in_chat {
            eprintln!("Failed to check if user is in chat: {}", e);
            return DatabaseGeneralResult_legacy::InternalError("Failed to check user in chat".to_string());
        }

        if let Ok(None) = user_in_chat {
            return DatabaseGeneralResult_legacy::NotFound;
        }

        let delete_messages_query = r#"
            DELETE FROM messages WHERE chat_id = ?;
        "#;

        let delete_messages_result = sqlx::query(delete_messages_query)
            .bind(chat_id)
            .execute(&self.database)
            .await;

        if let Err(e) = delete_messages_result {
            eprintln!("Failed to delete messages in chat: {}", e);
            return DatabaseGeneralResult_legacy::InternalError("Failed to delete messages in chat".to_string());
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

    pub async fn add_user_to_chat(&self, chat_id: i64, user_id: i64, key: &str, signature: &str) -> DatabaseGeneralResult_legacy<()> {
        if key.trim().is_empty() || signature.trim().is_empty() {
            return DatabaseGeneralResult_legacy::InvalidInput;
        }

        let chat_exists_query = "SELECT 1 FROM channels WHERE id = ?";
        let chat_exists = sqlx::query(chat_exists_query)
            .bind(chat_id)
            .fetch_optional(&self.database)
            .await;

        if let Err(e) = chat_exists {
            eprintln!("Failed to check if chat exists: {}", e);
            return DatabaseGeneralResult_legacy::InternalError("Failed to check if chat exists".to_string());
        }

        if let Ok(None) = chat_exists {
            return DatabaseGeneralResult_legacy::NotFound;
        }

        let check_query = r#"
            SELECT id FROM chat_users
            WHERE chat_id = ? AND user_id = ?;
        "#;

        let existing = sqlx::query(check_query)
            .bind(chat_id)
            .bind(user_id)
            .fetch_optional(&self.database)
            .await;

        match existing {
            Ok(Some(_)) => {
                return DatabaseGeneralResult_legacy::AlreadyExists;
            },
            Ok(None) => {},
            Err(e) => {
                eprintln!("Failed to check if user is already in chat: {}", e);
                return DatabaseGeneralResult_legacy::InternalError("Failed to check user in chat".to_string());
            }
        }

        let query = r#"
            INSERT INTO chat_users (chat_id, user_id, key, signature)
            VALUES (?, ?, ?, ?);
        "#;

        let result = sqlx::query(query)
            .bind(chat_id)
            .bind(user_id)
            .bind(key)
            .bind(signature)
            .execute(&self.database)
            .await;

        match result {
            Ok(_) => {
                DatabaseGeneralResult_legacy::Ok(())
            },
            Err(e) => {
                eprintln!("Failed to add user {} to chat {}: {}", user_id, chat_id, e);
                DatabaseGeneralResult_legacy::InternalError("Failed to add user to chat".to_string())
            }
        }
    }


    pub async fn get_chats_by_user_id(&self, user_id: i64) -> DatabaseGeneralResult_legacy<Vec<Chat>> {
        let query = r#"
            SELECT c.id, c.name, c.last_edited, cu.key, cu.signature
            FROM channels c
            INNER JOIN chat_users cu ON c.id = cu.chat_id
            WHERE cu.user_id = ?;
        "#;

        let rows = sqlx::query(query)
            .bind(user_id)
            .fetch_all(&self.database)
            .await;

        match rows {
            Ok(rows) => {
                let chats: Vec<Chat> = rows.into_iter().filter_map(|row| {
                    Some(Chat {
                        id: row.try_get("id").ok()?,
                        name: row.try_get("name").ok()?,
                        key: row.try_get("key").ok()?,
                        signature: row.try_get("signature").ok()?,
                    })
                }).collect();

                DatabaseGeneralResult_legacy::Ok(chats)

            },
            Err(e) => {
                eprintln!("Failed to fetch chats for user {}: {}", user_id, e);
                DatabaseGeneralResult_legacy::InternalError("Failed to fetch chats".to_string())
            }
        }
    }

    pub async fn remove_user_from_chat(&self, chat_id: i64, user_id: i64) -> DatabaseGeneralResult_legacy<()> {
        let check_query = r#"
            SELECT 1 FROM chat_users WHERE chat_id = ? AND user_id = ?;
        "#;

        let is_user = sqlx::query(check_query)
            .bind(chat_id)
            .bind(user_id)
            .fetch_optional(&self.database)
            .await;

        if let Err(e) = is_user {
            eprintln!("Failed to check if user {} is in chat {}: {}", user_id, chat_id, e);
            return DatabaseGeneralResult_legacy::InternalError("Failed to check user in chat".to_string());
        }

        if let Ok(None) = is_user {
            return DatabaseGeneralResult_legacy::NotFound;
        };

        let mut tx = match self.database.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("Failed to begin transaction for user removal from chat: {}", e);
                return DatabaseGeneralResult_legacy::InternalError("Failed to start transaction".to_string());
            }
        };

        let delete_messages_query = r#"
            DELETE FROM messages WHERE chat_id = ? AND user_id = ?;
        "#;
        if let Err(e) = sqlx::query(delete_messages_query)
            .bind(chat_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await
        {
            eprintln!("Failed to delete messages for user {} in chat {}: {}", user_id, chat_id, e);
            let rollback_result = tx.rollback().await;
            if let Err(e) = rollback_result {
                eprintln!("Cannot make rollback {e}")
            }
            return DatabaseGeneralResult_legacy::InternalError("Failed to delete user messages from chat".to_string());
        }

        let delete_chat_user_query = r#"
            DELETE FROM chat_users WHERE chat_id = ? AND user_id = ?;
        "#;
        if let Err(e) = sqlx::query(delete_chat_user_query)
            .bind(chat_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await
        {
            eprintln!("Failed to remove user {} from chat_users for chat {}: {}", user_id, chat_id, e);
            let rollback_result = tx.rollback().await;
            if let Err(e) = rollback_result {
                eprintln!("Cannot make rollback {e}")
            }
            return DatabaseGeneralResult_legacy::InternalError("Failed to remove user from chat".to_string());
        }

        let update_query = r#"
            UPDATE channels
            SET last_edited = strftime('%s', 'now')
            WHERE id = ?;
        "#;
        if let Err(e) = sqlx::query(update_query)
            .bind(chat_id)
            .execute(&mut *tx)
            .await
        {
            eprintln!("Failed to update last_edited after removing user {} from chat {}: {}", user_id, chat_id, e);
            let rollback_result = tx.rollback().await;
            if let Err(e) = rollback_result {
                eprintln!("Cannot make rollback {e}")
            }
            return DatabaseGeneralResult_legacy::InternalError("Failed to update channel timestamp".to_string());
        }

        match tx.commit().await {
            Ok(_) => DatabaseGeneralResult_legacy::Ok(()),
            Err(e) => {
                eprintln!("Failed to commit transaction for user removal from chat: {}", e);
                DatabaseGeneralResult_legacy::InternalError("Failed to commit user removal".to_string())
            }
        }
    }

    pub async fn add_message(&self, chat_id: i64, user_id: i64, content: &str) -> DatabaseGeneralResult_legacy<i64> {
        if content.trim().is_empty() {
            return DatabaseGeneralResult_legacy::InvalidInput;
        }
        let check_user_query = r#"
            SELECT 1 FROM chat_users WHERE chat_id = ? AND user_id = ?;
        "#;

        let user_in_chat = sqlx::query(check_user_query)
            .bind(chat_id)
            .bind(user_id)
            .fetch_optional(&self.database)
            .await;


        if let Err(e) = user_in_chat {
            eprintln!("Failed to check if user is in chat: {}", e);
            return DatabaseGeneralResult_legacy::InternalError("Failed to check user in chat".to_string());
        }

        if let Ok(None) = user_in_chat {
            return DatabaseGeneralResult_legacy::NotFound;
        }

        let message_query = r#"
            INSERT INTO messages (chat_id, user_id, content)
            VALUES (?, ?, ?)
            RETURNING id;
        "#;

        let message_result = sqlx::query(message_query)
            .bind(chat_id)
            .bind(user_id)
            .bind(content)
            .fetch_one(&self.database)
            .await;

        let message_id = match message_result {
            Ok(row) => row.get("id"),
            Err(e) => {
                eprintln!("Failed to insert message: {}", e);
                return DatabaseGeneralResult_legacy::InternalError("Failed to insert message".to_string());
            }
        };

        let update_query = r#"
            UPDATE channels
            SET last_edited = strftime('%s', 'now')
            WHERE id = ?;
        "#;

        let update_result = sqlx::query(update_query)
            .bind(chat_id)
            .execute(&self.database)
            .await;

        match update_result {
            Ok(_) => DatabaseGeneralResult_legacy::Ok(message_id),
            Err(e) => {
                eprintln!("Failed to update last_edited: {}", e);
                DatabaseGeneralResult_legacy::InternalError("Failed to update channel timestamp".to_string())
            }
        }
    }


    pub async fn remove_message(&self, message_id: i64, user_id: i64) -> DatabaseGeneralResult_legacy<()> {
        let check_query = r#"
            SELECT chat_id FROM messages WHERE id = ? AND user_id = ?;
        "#;

        let check_result = sqlx::query(check_query)
            .bind(message_id)
            .bind(user_id)
            .fetch_optional(&self.database)
            .await;

        let chat_id = match check_result {
            Ok(Some(row)) => row.get::<i64, _>("chat_id"),
            Ok(None) => return DatabaseGeneralResult_legacy::NotFound,
            Err(e) => {
                eprintln!("Failed to check message existence: {}", e);
                return DatabaseGeneralResult_legacy::InternalError("Failed to check message existence".to_string());
            }
        };

        let delete_query = r#"
            DELETE FROM messages WHERE id = ?;
        "#;

        let delete_result = sqlx::query(delete_query)
            .bind(message_id)
            .execute(&self.database)
            .await;

        match delete_result {
            Ok(_) => {
                let update_query = r#"
                    UPDATE channels
                    SET last_edited = strftime('%s', 'now')
                    WHERE id = ?;
                "#;

                let update_result = sqlx::query(update_query)
                    .bind(chat_id)
                    .execute(&self.database)
                    .await;

                match update_result {
                    Ok(_) => DatabaseGeneralResult_legacy::Ok(()),
                    Err(e) => {
                        eprintln!("Failed to update last_edited: {}", e);
                        DatabaseGeneralResult_legacy::InternalError("Failed to update channel timestamp".to_string())
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to delete message: {}", e);
                DatabaseGeneralResult_legacy::InternalError("Failed to delete message".to_string())
            }
        }
    }

    pub async fn register_user(&self, username: &str, public_key: &str) -> DatabaseGeneralResult_legacy<()> {
        if username.trim().is_empty() {
            return DatabaseGeneralResult_legacy::InvalidInput;
        }

        let query = r#"
            INSERT INTO users (username, public_key)
            VALUES (?, ?);
        "#;

        let result = sqlx::query(query)
            .bind(username)
            .bind(public_key)
            .execute(&self.database)
            .await;

        match result {
            Ok(_) => {
                println!("User '{}' registered", username);
                DatabaseGeneralResult_legacy::Ok(())
            },
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.message().contains("UNIQUE constraint failed") {
                        return DatabaseGeneralResult_legacy::AlreadyExists;
                    }
                }
                eprintln!("Database error: {}", e);
                DatabaseGeneralResult_legacy::InternalError("Database error".to_string())
            }
        }
    }

    async fn get_user_by_query<T>(&self, query: &str, bind_value: T) -> DatabaseGeneralResult_legacy<User>
    where
        T: for<'q> sqlx::Encode<'q, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite> + Send,
    {
        let row = match sqlx::query(query)
            .bind(bind_value)
            .fetch_optional(&self.database)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Database query failed: {}", e);
                return DatabaseGeneralResult_legacy::InternalError("Internal server error".to_string());
            }
        };

        match row {
            Some(row) => {
                let id: i64 = match row.try_get("id") {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to extract id: {}", e);
                        return DatabaseGeneralResult_legacy::InternalError("Internal server error".to_string());
                    }
                };
                let public_key: String = match row.try_get("public_key") {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to extract public_key: {}", e);
                        return DatabaseGeneralResult_legacy::InternalError("Internal server error".to_string());
                    }
                };
                DatabaseGeneralResult_legacy::Ok(User { id, public_key })
            }
            None => DatabaseGeneralResult_legacy::NotFound,
        }
    }

    pub async fn get_user_by_username(&self, username: &str) -> DatabaseGeneralResult_legacy<User> {
        self.get_user_by_query(
            "SELECT id, username, public_key FROM users WHERE username = ?;",
            username.to_string(),
        ).await
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> DatabaseGeneralResult_legacy<User> {
        self.get_user_by_query(
            "SELECT id, username, public_key FROM users WHERE id = ?;",
            &user_id,
        ).await
    }

    pub async fn get_user_public_key(&self, username: &str) -> DatabaseGeneralResult_legacy<String> {
        match self.get_user_by_username(username).await {
            DatabaseGeneralResult_legacy::Ok(user) => DatabaseGeneralResult_legacy::Ok(user.public_key),
            DatabaseGeneralResult_legacy::NotFound => DatabaseGeneralResult_legacy::NotFound,
            DatabaseGeneralResult_legacy::AlreadyExists => DatabaseGeneralResult_legacy::AlreadyExists,
            DatabaseGeneralResult_legacy::InvalidInput => DatabaseGeneralResult_legacy::InvalidInput,
            DatabaseGeneralResult_legacy::InternalError(e) => DatabaseGeneralResult_legacy::InternalError(e)
        }
    }

    pub async fn get_user_id_by_username(&self, username: &str) -> DatabaseGeneralResult_legacy<i64> {
        match self.get_user_by_username(username).await {
            DatabaseGeneralResult_legacy::Ok(user) => DatabaseGeneralResult_legacy::Ok(user.id),
            DatabaseGeneralResult_legacy::NotFound => DatabaseGeneralResult_legacy::NotFound,
            DatabaseGeneralResult_legacy::AlreadyExists => DatabaseGeneralResult_legacy::AlreadyExists,
            DatabaseGeneralResult_legacy::InvalidInput => DatabaseGeneralResult_legacy::InvalidInput,
            DatabaseGeneralResult_legacy::InternalError(e) => DatabaseGeneralResult_legacy::InternalError(e)
        }
    }

    pub async fn get_messages(&self, chat_id: i64, user_id: i64, before_message_id: Option<i64>, ) -> DatabaseGeneralResult_legacy<Vec<Message>> {
        let limit = 50;

        let check_user_query = r#"
            SELECT 1 FROM chat_users WHERE chat_id = ? AND user_id = ?;
        "#;
        match sqlx::query(check_user_query)
            .bind(chat_id)
            .bind(user_id)
            .fetch_optional(&self.database)
            .await
        {
            Ok(Some(_)) => { },
            Ok(None) => {
                return DatabaseGeneralResult_legacy::NotFound;
            }
            Err(e) => {
                eprintln!(
                    "Failed to check if user {} is in chat {} before getting messages: {}",
                    user_id, chat_id, e
                );
                return DatabaseGeneralResult_legacy::InternalError(
                    "Failed to verify user chat membership".to_string(),
                );
            }
        }

        let query_str: String;
        let query_result = match before_message_id {
            Some(before_id) => {
                query_str = r#"
                    SELECT id, chat_id, user_id, content, timestamp
                    FROM messages
                    WHERE chat_id = ? AND id < ?
                    ORDER BY id DESC
                    LIMIT ?;
                "#.to_string();
                sqlx::query(&query_str)
                    .bind(chat_id)
                    .bind(before_id)
                    .bind(limit)
                    .fetch_all(&self.database)
                    .await
            }
            None => {
                query_str = r#"
                    SELECT id, chat_id, user_id, content, timestamp
                    FROM messages
                    WHERE chat_id = ?
                    ORDER BY id DESC
                    LIMIT ?;
                "#.to_string();
                sqlx::query(&query_str)
                    .bind(chat_id)
                    .bind(limit)
                    .fetch_all(&self.database)
                    .await
            }
        };

        match query_result {
            Ok(rows) => {
                let mut messages: Vec<Message> = rows
                    .into_iter()
                    .map(|row| Message {
                        id: row.get("id"),
                        chat_id: row.get("chat_id"),
                        user_id: row.get("user_id"),
                        content: row.get("content"),
                        timestamp: row.get("timestamp"),
                    })
                    .collect();
                messages.reverse();

                DatabaseGeneralResult_legacy::Ok(messages)
            }
            Err(sqlx::Error::RowNotFound) => {
                DatabaseGeneralResult_legacy::Ok(Vec::new())
            }
            Err(e) => {
                eprintln!("Failed to fetch messages for chat {}: {}", chat_id, e);
                DatabaseGeneralResult_legacy::InternalError("Failed to fetch messages".to_string())
            }
        }
    }
}