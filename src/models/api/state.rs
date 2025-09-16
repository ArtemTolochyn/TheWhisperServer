#[derive(Clone)]
pub struct Session {
    pub id: i64,
    tokens: Vec<String>
}

impl Session
{
    pub fn new(id: i64, token: String) -> Self
    {
        Self
        {
            id,
            tokens: vec![token],
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