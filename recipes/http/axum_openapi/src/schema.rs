use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostUser {
    pub user_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn mock() -> Self {
        let id = Uuid::now_v7();
        let now = Utc::now();

        Self {
            id: id,
            user_name: format!("user_{id}"),
            email: format!("{id}@test.com"),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct QueryUser {
    pub id: Option<Uuid>,
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
