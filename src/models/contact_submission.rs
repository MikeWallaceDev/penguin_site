use chrono::prelude::*;
#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ContactSubmission {
    uuid: Uuid,
    name: String,
    email: String,
    message: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl ContactSubmission {
    pub fn new(
        uuid: Uuid,
        name: String,
        email: String,
        message: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            uuid,
            name,
            email,
            message,
            created_at,
            updated_at,
        }
    }

    pub fn new_add(name: String, email: String, message: String) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name,
            email,
            message,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    #[inline]
    pub fn id(&self) -> String {
        self.uuid.to_string()
    }

    #[inline]
    pub fn email(&self) -> String {
        self.email.to_string()
    }

    #[inline]
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    #[inline]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    #[inline]
    pub fn create_date(&self) -> NaiveDateTime {
        self.created_at.clone()
    }

    #[inline]
    pub fn update_date(&self) -> NaiveDateTime {
        self.updated_at.clone()
    }
}

impl ContactSubmission {
    #[tracing::instrument]
    #[cfg(feature = "ssr")]
    pub async fn add(&self) -> Result<(), ServerFnError> {
        tracing::info!("Saving contact submission from: {}", self.email);

        sqlx::query!(
            "INSERT INTO contact_submissions (uuid, name, email, message) VALUES (?, ?, ?, ?)",
            self.id(),
            self.name,
            self.email,
            self.message
        )
        .execute(crate::database::get_db())
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        Ok(())
    }

    #[tracing::instrument]
    #[cfg(feature = "ssr")]
    pub async fn delete(id: String) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM contact_submissions WHERE uuid=$1", id,)
            .execute(crate::database::get_db())
            .await
    }

    #[tracing::instrument]
    #[cfg(feature = "ssr")]
    pub async fn get(id: String) -> Result<ContactSubmission, ServerFnError> {
        let row = sqlx::query!("SELECT * FROM contact_submissions WHERE uuid=$1", id,)
            .fetch_one(crate::database::get_db())
            .await
            .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        let uuid = Uuid::parse_str(&row.uuid)
            .map_err(|e| ServerFnError::new(format!("Invalid UUID in database: {}", e)))?;

        Ok(ContactSubmission {
            uuid,
            name: row.name,
            email: row.email,
            message: row.message,
            created_at: row.created_at.unwrap_or_default(),
            updated_at: row.updated_at.unwrap_or_default(),
        })
    }
}
