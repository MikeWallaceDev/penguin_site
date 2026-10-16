use serde::{Deserialize, Serialize};

pub const NAME_MIN: usize = 3;
pub const NAME_MAX: usize = 50;
pub const EMAIL_MAX: usize = 254;
pub const MESSAGE_MIN: usize = 20;
pub const MESSAGE_MAX: usize = 1000;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    Name,
    Email,
    Message,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ValidationCode {
    Required,
    TooShort,
    TooLong,
    InvalidEmail,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct FieldError {
    pub field: Field,
    pub code: ValidationCode,
}

impl FieldError {
    pub fn new(field: Field, code: ValidationCode) -> Self {
        Self { field, code }
    }
}

/// Structured response returned by the `SaveContact` server function.
///
/// Validation failures are reported as `Invalid` (not as an error) so the
/// client can map them back to individual fields with localized messages.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ContactResponse {
    Ok,
    Invalid(Vec<FieldError>),
}

pub fn validate_name(value: &str) -> Option<ValidationCode> {
    let value = value.trim();
    if value.is_empty() {
        return Some(ValidationCode::Required);
    }
    let len = value.chars().count();
    if len < NAME_MIN {
        return Some(ValidationCode::TooShort);
    }
    if len > NAME_MAX {
        return Some(ValidationCode::TooLong);
    }
    None
}

pub fn validate_email(value: &str) -> Option<ValidationCode> {
    let value = value.trim();
    if value.is_empty() {
        return Some(ValidationCode::Required);
    }
    if value.chars().count() > EMAIL_MAX {
        return Some(ValidationCode::InvalidEmail);
    }
    if value.chars().any(char::is_whitespace) {
        return Some(ValidationCode::InvalidEmail);
    }

    let mut parts = value.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = match parts.next() {
        Some(domain) => domain,
        None => return Some(ValidationCode::InvalidEmail),
    };
    if parts.next().is_some() {
        return Some(ValidationCode::InvalidEmail);
    }

    if local.is_empty() || local.chars().count() > 64 {
        return Some(ValidationCode::InvalidEmail);
    }
    if domain.is_empty()
        || !domain.contains('.')
        || domain.starts_with('.')
        || domain.ends_with('.')
        || domain.contains("..")
    {
        return Some(ValidationCode::InvalidEmail);
    }

    let tld = domain.rsplit('.').next().unwrap_or_default();
    if tld.chars().count() < 2 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return Some(ValidationCode::InvalidEmail);
    }

    let allowed = |c: char| {
        c.is_ascii_alphanumeric() || matches!(c, '@' | '.' | '_' | '-' | '+')
    };
    if !value.chars().all(allowed) {
        return Some(ValidationCode::InvalidEmail);
    }

    None
}

pub fn validate_message(value: &str) -> Option<ValidationCode> {
    let value = value.trim();
    if value.is_empty() {
        return Some(ValidationCode::Required);
    }
    let len = value.chars().count();
    if len < MESSAGE_MIN {
        return Some(ValidationCode::TooShort);
    }
    if len > MESSAGE_MAX {
        return Some(ValidationCode::TooLong);
    }
    None
}

/// Validates every field and returns all errors (empty if the input is valid).
pub fn validate_contact(name: &str, email: &str, message: &str) -> Vec<FieldError> {
    let mut errors = Vec::new();

    if let Some(code) = validate_name(name) {
        errors.push(FieldError::new(Field::Name, code));
    }
    if let Some(code) = validate_email(email) {
        errors.push(FieldError::new(Field::Email, code));
    }
    if let Some(code) = validate_message(message) {
        errors.push(FieldError::new(Field::Message, code));
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_rules() {
        assert_eq!(validate_name(""), Some(ValidationCode::Required));
        assert_eq!(validate_name("  "), Some(ValidationCode::Required));
        assert_eq!(validate_name("Jo"), Some(ValidationCode::TooShort));
        assert_eq!(validate_name("Joe"), None);
        assert_eq!(
            validate_name(&"a".repeat(NAME_MAX + 1)),
            Some(ValidationCode::TooLong)
        );
    }

    #[test]
    fn email_rules() {
        assert_eq!(validate_email(""), Some(ValidationCode::Required));
        assert_eq!(validate_email("nope"), Some(ValidationCode::InvalidEmail));
        assert_eq!(
            validate_email("a@b"),
            Some(ValidationCode::InvalidEmail)
        );
        assert_eq!(
            validate_email("a@b.c"),
            Some(ValidationCode::InvalidEmail)
        );
        assert_eq!(validate_email("user@example.com"), None);
        assert_eq!(validate_email("first.last+tag@sub.example.co"), None);
        assert_eq!(
            validate_email("a b@example.com"),
            Some(ValidationCode::InvalidEmail)
        );
    }

    #[test]
    fn message_rules() {
        assert_eq!(validate_message(""), Some(ValidationCode::Required));
        assert_eq!(validate_message("too short"), Some(ValidationCode::TooShort));
        assert_eq!(validate_message(&"x".repeat(MESSAGE_MIN)), None);
        assert_eq!(
            validate_message(&"x".repeat(MESSAGE_MAX + 1)),
            Some(ValidationCode::TooLong)
        );
    }

    #[test]
    fn contact_collects_all_errors() {
        let errors = validate_contact("Jo", "bad", "short");
        assert_eq!(errors.len(), 3);
        assert!(errors.iter().any(|e| e.field == Field::Name));
        assert!(errors.iter().any(|e| e.field == Field::Email));
        assert!(errors.iter().any(|e| e.field == Field::Message));
    }
}
