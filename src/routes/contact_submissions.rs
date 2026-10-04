#[cfg(feature = "ssr")]
use crate::models::ContactSubmission;
use leptos::prelude::*;

#[tracing::instrument]
#[server(SaveContact, "/api")]
pub async fn save_contact_submission(
    name: String,
    email: String,
    message: String,
) -> Result<(), ServerFnError> {
    tracing::info!("Saving contact submission from: {}", email);

    let contact_submission_add = ContactSubmission::new_add(name, email, message);
    contact_submission_add.add().await
}
