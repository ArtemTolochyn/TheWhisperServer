use sqlx::{FromRow, Row, Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use crate::DB_URL;
use serde::Serialize;
use crate::database::channel::{add_user_to_chat, create_chat, get_channels_by_user_id, is_user_in_chat, remove_chat, remove_user_from_chat};
use crate::database::initialization::{create_channels_table, create_chat_users_table, create_messages_table, create_users_table};
use crate::database::message::{add_message, get_messages};
use crate::database::user::{get_user_by_id, get_user_by_username, register_user};

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



#[derive(Debug, Serialize, FromRow)]
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

    //User.rs
    pub async fn get_user_by_username(&self, username: &str) -> Result<User, DatabaseGeneralError> {
        get_user_by_username(&self.database, username).await
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, DatabaseGeneralError> {
        get_user_by_id(&self.database, user_id).await
    }

    pub async fn register_user(&self, username: &str, public_key: &str) -> Result<(), DatabaseGeneralError> {
        register_user(&self.database, username, public_key).await
    }


    //Chat.rs
    pub async fn is_user_in_chat(&self, user_id: i64, chat_id: i64) -> Result<bool, String> {
        is_user_in_chat(&self.database, user_id, chat_id).await
    }

    pub async fn create_chat(&self, name: &str) -> Result<ChatInfo, DatabaseGeneralError> {
        create_chat(&self.database, name).await
    }

    pub async fn remove_chat(&self, chat_id: i64, user_id: i64) -> Result<(), DatabaseGeneralError> {
        remove_chat(&self.database, chat_id, user_id).await
    }

    pub async fn add_user_to_chat(&self, chat_id: i64, user_id: i64, key: &str, signature: &str) -> Result<(), DatabaseGeneralError> {
        add_user_to_chat(&self.database, chat_id, user_id, key, signature).await
    }

    pub async fn get_channels_by_user_id(&self, user_id: i64) -> Result<Vec<Chat>, DatabaseGeneralError> {
        get_channels_by_user_id(&self.database, user_id).await
    }

    pub async fn remove_user_from_chat(&self, chat_id: i64, user_id: i64) -> Result<(), DatabaseGeneralError> {
        remove_user_from_chat(&self.database, chat_id, user_id).await
    }

    //Message.rs
    pub async fn add_message(&self, chat_id: i64, user_id: i64, content: &str) -> Result<i64, DatabaseGeneralError> {
        add_message(&self.database, chat_id, user_id, content).await
    }

    pub async fn remove_message(&self, message_id: i64, user_id: i64) -> Result<(), DatabaseGeneralError> {
        remove_chat(&self.database, message_id, user_id).await
    }

    pub async fn get_messages(&self, chat_id: i64, user_id: i64, before_message_id: Option<i64>) -> Result<Vec<Message>, DatabaseGeneralError> {
        get_messages(&self.database, chat_id, user_id, before_message_id).await
    }
}