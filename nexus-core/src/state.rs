use sqlx::{Pool, Postgres};
use crate::clients::{OpenAiClient, OpenRouterClient};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub openrouter: Option<OpenRouterClient>,
    pub openai: Option<OpenAiClient>,
}
