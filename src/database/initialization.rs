use sqlx::{SqlitePool};

pub async fn create_users_table(database: &SqlitePool) -> Result<(), String> {
    sqlx::query("
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                public_key TEXT NOT NULL
            );
        ")
        .execute(database)
        .await
        .map_err(|e| format!("Failed to create users table: {}", e))?;

    println!("Users table ensured");
    Ok(())
}

pub async fn create_channels_table(database: &SqlitePool) -> Result<(), String> {
    sqlx::query("
            CREATE TABLE IF NOT EXISTS channels (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            );
        ")
        .execute(database)
        .await
        .map_err(|e| format!("Failed to create channels table: {}", e))?;

    println!("Channels table ensured");
    Ok(())
}

pub async fn create_chat_users_table(database: &SqlitePool) -> Result<(), String> {
    sqlx::query("
            CREATE TABLE IF NOT EXISTS chat_users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                chat_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                key TEXT NOT NULL,
                signature TEXT NOT NULL,
                FOREIGN KEY (chat_id) REFERENCES channels(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );
        ")
        .execute(database)
        .await
        .map_err(|e| format!("Failed to create chat_users table: {}", e))?;

    println!("Chat users table ensured");
    Ok(())
}

pub async fn create_messages_table(database: &SqlitePool) -> Result<(), String> {
    sqlx::query("
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                chat_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                content TEXT NOT NULL,
                timestamp INTEGER NOT NULL DEFAULT (crtime('%s', 'now')),
                FOREIGN KEY (chat_id) REFERENCES channels(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );
        ")
        .execute(database)
        .await
        .map_err(|e| format!("Failed to create messages table: {}", e))?;

    println!("Messages table ensured");
    Ok(())
}