//! Email notifications for contact-form submissions.
//!
//! Delivery uses an external SMTP provider (Gmail, Resend, Mailgun, ...), so no
//! mail daemon runs on this server. Configuration comes from the environment:
//!
//! | Variable        | Required | Notes                                              |
//! |-----------------|----------|----------------------------------------------------|
//! | `SMTP_HOST`     | yes      | e.g. `smtp.resend.com`. Unset = notifications off   |
//! | `SMTP_PORT`     | no       | default `587`                                       |
//! | `SMTP_USERNAME` | usually  | skipped when empty                                  |
//! | `SMTP_PASSWORD` | usually  | skipped when empty                                  |
//! | `NOTIFY_FROM`   | no       | default `no-reply@pingouin-techno.com`              |
//! | `NOTIFY_TO`     | no       | default `penguin@nopez.addy.io`                     |
//!
//! Transport security is picked from the port: `465` uses implicit TLS,
//! `25` sends in plaintext, and every other port (587 by default) uses
//! STARTTLS.

#[cfg(feature = "ssr")]
use std::time::Duration;

/// Upper bound on one SMTP round-trip so a slow provider cannot stall the
/// contact request for long.
#[cfg(feature = "ssr")]
const SEND_TIMEOUT: Duration = Duration::from_secs(20);

#[cfg(feature = "ssr")]
const DEFAULT_SMTP_PORT: u16 = 587;
#[cfg(feature = "ssr")]
const DEFAULT_FROM: &str = "no-reply@pingouin-techno.com";
#[cfg(feature = "ssr")]
const DEFAULT_TO: &str = "penguin@nopez.addy.io";

/// Notifies the site owner that somebody submitted the contact form.
///
/// The SMTP round-trip can take seconds, so callers should run this on a
/// background task rather than awaiting it in the request handler.
///
/// Returns `Ok(())` when notifications are not configured, so an unconfigured
/// environment keeps working exactly as before.
#[cfg(feature = "ssr")]
pub async fn notify_new_contact(name: &str, email: &str, message: &str) -> Result<(), String> {
   use lettre::Transport;

   let config = match SmtpConfig::from_env() {
      Ok(config) => config,
      Err(reason) => {
         tracing::info!("Contact notification skipped: {reason}");
         return Ok(());
      }
   };

   let mail = build_message(&config, name, email, message)?;
   let transport = build_transport(&config)?;

   let response = tokio::time::timeout(
      SEND_TIMEOUT,
      tokio::task::spawn_blocking(move || transport.send(&mail)),
   )
   .await
   .map_err(|_| format!("SMTP send timed out after {SEND_TIMEOUT:?}"))?
   .map_err(|err| format!("notification task failed: {err}"))?;

   match response {
      Ok(response) => {
         tracing::info!("Contact notification sent (code {})", response.code());
         Ok(())
      }
      Err(err) => Err(format!("SMTP delivery failed: {err}")),
   }
}

#[cfg(not(feature = "ssr"))]
pub async fn notify_new_contact(_name: &str, _email: &str, _message: &str) -> Result<(), String> {
   Ok(())
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
struct SmtpConfig {
   host: String,
   port: u16,
   username: String,
   password: String,
   from: String,
   to: String,
}

#[cfg(feature = "ssr")]
impl SmtpConfig {
   fn from_env() -> Result<Self, String> {
      let host = env_value("SMTP_HOST").ok_or("SMTP_HOST is not set")?;

      let port = match env_value("SMTP_PORT") {
         Some(raw) => raw
            .parse::<u16>()
            .map_err(|_| format!("SMTP_PORT is not a valid port: {raw}"))?,
         None => DEFAULT_SMTP_PORT,
      };

      Ok(Self {
         host,
         port,
         username: env_value("SMTP_USERNAME").unwrap_or_default(),
         password: env_value("SMTP_PASSWORD").unwrap_or_default(),
         from: env_value("NOTIFY_FROM").unwrap_or_else(|| DEFAULT_FROM.to_string()),
         to: env_value("NOTIFY_TO").unwrap_or_else(|| DEFAULT_TO.to_string()),
      })
   }
}

/// Reads an environment variable, treating blank values as unset.
#[cfg(feature = "ssr")]
fn env_value(key: &str) -> Option<String> {
   std::env::var(key)
      .ok()
      .map(|value| value.trim().to_string())
      .filter(|value| !value.is_empty())
}

/// Strips CR/LF and other control characters so user input cannot inject
/// extra headers when it is used in the subject line.
#[cfg(feature = "ssr")]
fn sanitize_header(value: &str) -> String {
   value
      .chars()
      .map(|c| if c.is_control() { ' ' } else { c })
      .collect::<String>()
      .split_whitespace()
      .collect::<Vec<_>>()
      .join(" ")
}

#[cfg(feature = "ssr")]
fn build_message(
   config: &SmtpConfig,
   name: &str,
   email: &str,
   message: &str,
) -> Result<lettre::Message, String> {
   let from: lettre::message::Mailbox = config
      .from
      .parse()
      .map_err(|err| format!("NOTIFY_FROM is not a valid address: {err}"))?;

   let to: lettre::message::Mailbox = config
      .to
      .parse()
      .map_err(|err| format!("NOTIFY_TO is not a valid address: {err}"))?;

   // Validated by `validate_email`, so it carries no header separators.
   let reply_to: lettre::message::Mailbox = email
      .parse()
      .map_err(|err| format!("submission email is not a valid address: {err}"))?;

   let subject = format!(
      "🐧 Penguin - New contact submission from : {}",
      sanitize_header(name)
   );

   // A trailing `\` continues the literal on the next line and drops that
   // line's leading whitespace, so this stays readable in the source while
   // the email body keeps no indentation or markers.
   let body = format!(
      "Received a new contact submission\n\n\
       Name:\n  {name}\n\
       Email:\n {email}\n\n\
       Message:\n{message}\n\n\
       -MIke\n"
   );

   lettre::Message::builder()
      .from(from)
      .to(to)
      .reply_to(reply_to)
      .subject(subject)
      .body(body)
      .map_err(|err| format!("failed to build notification email: {err}"))
}

/// How the connection to the relay is secured.
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TlsMode {
   /// Implicit TLS (SMTPS) on the connection itself, used on port 465.
   ImplicitTls,
   /// Plaintext connection upgraded with STARTTLS, used on port 587 and most
   /// other submission ports.
   StartTls,
   /// Plaintext connection with no encryption, used on port 25.
   None,
}

#[cfg(feature = "ssr")]
fn tls_mode_for(port: u16) -> TlsMode {
   match port {
      465 => TlsMode::ImplicitTls,
      25 => TlsMode::None,
      _ => TlsMode::StartTls,
   }
}

#[cfg(feature = "ssr")]
fn build_transport(config: &SmtpConfig) -> Result<lettre::transport::smtp::SmtpTransport, String> {
   use lettre::transport::smtp::SmtpTransport;
   use lettre::transport::smtp::authentication::Credentials;

   let tls_mode = tls_mode_for(config.port);
   tracing::debug!(
      "Opening SMTP connection to {}:{} ({:?})",
      config.host,
      config.port,
      tls_mode
   );

   let mut builder = match tls_mode {
      TlsMode::ImplicitTls => SmtpTransport::relay(&config.host)
         .map_err(|err| format!("cannot configure relay {}: {err}", config.host))?,
      TlsMode::StartTls => SmtpTransport::starttls_relay(&config.host)
         .map_err(|err| format!("cannot configure relay {}: {err}", config.host))?,
      // No TLS: the relay is reached directly on the given port.
      TlsMode::None => SmtpTransport::builder_dangerous(&config.host),
   }
   .port(config.port);

   if !config.username.is_empty() {
      builder = builder.credentials(Credentials::new(
         config.username.clone(),
         config.password.clone(),
      ));
   }

   Ok(builder.build())
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
   use super::*;

   #[test]
   fn subject_strips_header_injection() {
      assert_eq!(
         sanitize_header("Ann\nBcc: evil@x.com"),
         "Ann Bcc: evil@x.com"
      );
      assert_eq!(sanitize_header("  Ada  "), "Ada");
   }

   #[test]
   fn tls_mode_follows_port() {
      assert_eq!(tls_mode_for(25), TlsMode::None);
      assert_eq!(tls_mode_for(587), TlsMode::StartTls);
      assert_eq!(tls_mode_for(2525), TlsMode::StartTls);
      assert_eq!(tls_mode_for(465), TlsMode::ImplicitTls);
   }

   #[test]
   fn builds_notification_message() {
      let config = SmtpConfig {
         host: "smtp.example.com".into(),
         port: 587,
         username: String::new(),
         password: String::new(),
         from: DEFAULT_FROM.into(),
         to: DEFAULT_TO.into(),
      };

      let mail = build_message(
         &config,
         "Ada",
         "ada@example.com",
         "Hello there, this is fine.",
      )
      .expect("message should build");

      let rendered = String::from_utf8_lossy(&mail.formatted()).to_string();
      // Read back the decoded header: the emoji makes the raw form an
      // RFC2047 encoded-word.
      let subject = mail
         .headers()
         .get::<lettre::message::header::Subject>()
         .expect("message should carry a subject");
      assert_eq!(
         subject.as_ref(),
         "🐧 Penguin - New contact submission from : Ada"
      );
      assert!(rendered.contains("Reply-To: ada@example.com"), "{rendered}");
      assert!(rendered.contains("Hello there, this is fine."));
      assert!(rendered.contains("Name:  Ada"), "{rendered}");
      assert!(rendered.contains("-MIke"), "{rendered}");
      // The body is written across several source lines; its layout markers
      // and indentation must never reach the email.
      assert!(
         !rendered.contains(" / "),
         "marker leaked into body: {rendered}"
      );
   }
}
