use sqlx::{SqlitePool};

pub async fn create_users_table(database: &SqlitePool) -> Result<(), String> {
    let query = r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                public_key TEXT NOT NULL
            );
        "#;

    sqlx::query(query)
        .execute(database)
        .await
        .map_err(|_| "Failed to create users table")?;

    println!("Users table ensured");
    Ok(())
}

pub async fn create_channels_table(database: &SqlitePool) -> Result<(), String> {
    let query = r#"
            CREATE TABLE IF NOT EXISTS channels (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                last_edited INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );
        "#;

    sqlx::query(query)
        .execute(database)
        .await
        .map_err(|_| "Failed to create channels table")?;

    println!("Channels table ensured");
    Ok(())
}

pub async fn create_chat_users_table(database: &SqlitePool) -> Result<(), String> {
    let query = r#"
            CREATE TABLE IF NOT EXISTS chat_users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                chat_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                key TEXT NOT NULL,
                signature TEXT NOT NULL,
                FOREIGN KEY (chat_id) REFERENCES channels(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );
        "#;

    sqlx::query(query)
        .execute(database)
        .await
        .map_err(|_| "Failed to create chat_users table")?;

    println!("Chat users table ensured");
    Ok(())
}

pub async fn create_messages_table(database: &SqlitePool) -> Result<(), String> {
    let query = r#"
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                chat_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                content TEXT NOT NULL,
                timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                FOREIGN KEY (chat_id) REFERENCES channels(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );
        "#;

    sqlx::query(query)
        .execute(database)
        .await
        .map_err(|_| "Failed to create messages table")?;

    println!("Messages table ensured");
    Ok(())
}