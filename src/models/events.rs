use std::sync::{Arc, Mutex};
use actix_ws::{Session};

#[derive(Clone)]
pub struct UserSessionsData {
    pub id: i64,
    tokens: Vec<String>,
    streams: Vec<Arc<Mutex<Session>>>
}

impl UserSessionsData
{
    pub fn new(id: i64, token: String) -> Self
    {
        Self
        {
            id,
            tokens: vec![token],
            streams: Vec::new()
        }
    }

    pub fn add_token(&mut self, token: String) {
        self.tokens.push(token);
    }

    pub fn check_token(&self, token: String) -> bool
    {
        self.tokens.contains(&token)
    }
}
