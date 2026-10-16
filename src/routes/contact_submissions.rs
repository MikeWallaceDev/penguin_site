#[cfg(feature = "ssr")]
use crate::models::ContactSubmission;
use crate::validation::ContactResponse;
use leptos::prelude::*;

#[server(SaveContact, "/api")]
pub async fn save_contact_submission(
    name: String,
    email: String,
    message: String,
) -> Result<ContactResponse, ServerFnError> {
    let name = name.trim().to_string();
    let email = email.trim().to_string();
    let message = message.trim().to_string();

    tracing::info!("Saving contact submission from: {}", email);

    let errors = crate::validation::validate_contact(&name, &email, &message);
    if !errors.is_empty() {
        tracing::warn!(
            "Rejected contact submission: {} validation error(s)",
            errors.len()
        );
        return Ok(ContactResponse::Invalid(errors));
    }

    let contact_submission_add = ContactSubmission::new_add(name, email, message);
    contact_submission_add.add().await?;

    Ok(ContactResponse::Ok)
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::validation::Field;

    #[tokio::test]
    async fn rejects_invalid_submission_before_db() {
        let response = save_contact_submission(
            "Jo".to_string(),
            "not-an-email".to_string(),
            "short".to_string(),
        )
        .await
        .expect("server fn should not error on validation failure");

        match response {
            ContactResponse::Invalid(errors) => {
                assert_eq!(errors.len(), 3);
                assert!(errors.iter().any(|error| error.field == Field::Name));
                assert!(errors.iter().any(|error| error.field == Field::Email));
                assert!(errors.iter().any(|error| error.field == Field::Message));
            }
            ContactResponse::Ok => panic!("expected invalid response"),
        }
    }
}
