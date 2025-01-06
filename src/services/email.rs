use crate::utils::Email;
use lettre::{
    message::{header::ContentType, Attachment, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::time::Duration;
use thiserror::Error;

#[cfg(not(debug_assertions))]
use lettre::transport::smtp::authentication::Credentials;

#[derive(Error, Debug)]
pub enum MailerError {
    #[error("Failed to send email")]
    SmtpSendError,

    #[error("Failed to parse origin email")]
    InvalidOriginEmail,

    #[error("Failed to parse recipient email")]
    InvalidRecipientEmail,

    #[error("Failed to build email")]
    BuildEmailError,

    #[error("Invalid attachment")]
    InvalidAttachment,
}

#[async_trait::async_trait]
pub trait MailerTrait {
    fn new(smtp_username: String, smtp_password: String, smtp_host: &str, origin_email: String, smtp_port: u16) -> Self;
    /// Creates a mail.
    ///
    /// # Arguments
    ///
    /// * `email` - The email to send
    ///
    /// # Returns
    ///
    /// Returns a `Message` if the email is valid.
    ///
    /// # Errors
    ///
    /// Returns `MailerError::InvalidOriginEmail` if the origin email is invalid.
    /// Returns `MailerError::InvalidRecipientEmail` if the recipient email is invalid.
    /// Returns `MailerError::BuildEmailError` if the email cannot be built.
    fn create_mail(&self, email: Email) -> Result<Message, MailerError>;

    /// Sends a mail.
    async fn send_mail(&self, email: Message) -> Result<(), MailerError>;
}

#[derive(Clone)]
pub struct Mailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    origin_email: String,
}

#[async_trait::async_trait]
impl MailerTrait for Mailer {
    /// Creates a new mailer.
    ///
    /// # Panics
    ///
    /// Panics if the mailer cannot be created.
    ///
    /// # Errors
    #[allow(unused_variables)]
    fn new(smtp_username: String, smtp_password: String, smtp_host: &str, origin_email: String, smtp_port: u16) -> Self {
        #[cfg(not(debug_assertions))]
        let credentials = Credentials::new(smtp_username, smtp_password);

        #[cfg(not(debug_assertions))]
        let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
            .unwrap()
            .credentials(credentials)
            .timeout(Some(Duration::from_secs(10)))
            .build();

        // For local development, this will not be compiled on release mode and E2E tests
        #[cfg(debug_assertions)]
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(smtp_host)
            .port(smtp_port)
            .timeout(Some(Duration::from_secs(10)))
            .build();

        Self { mailer, origin_email }
    }

    /// Creates a mail.
    ///
    /// # Arguments
    ///
    /// * `email` - The email to send
    ///
    /// # Returns
    ///
    /// Returns a `Message` if the email is valid.
    ///
    /// # Errors
    ///
    /// Returns `MailerError::InvalidOriginEmail` if the origin email is invalid.
    /// Returns `MailerError::InvalidRecipientEmail` if the recipient email is invalid.
    /// Returns `MailerError::BuildEmailError` if the email cannot be built.
    fn create_mail(&self, email: Email) -> Result<Message, MailerError> {
        let html_part = SinglePart::builder().header(ContentType::TEXT_HTML).body(email.html_body);

        let mut email_parts = MultiPart::mixed().singlepart(html_part);

        // Attachments can only be PDF and named invoice
        if let Some(attachment) = email.attachment {
            let content_type = ContentType::parse("application/pdf").map_err(|_| MailerError::InvalidAttachment)?;

            email_parts = email_parts.singlepart(Attachment::new(String::from("invoice.pdf")).body(attachment, content_type));
        }

        Message::builder()
            .from(self.origin_email.parse().map_err(|_| MailerError::InvalidOriginEmail)?)
            .to(email.to.parse().map_err(|_| MailerError::InvalidRecipientEmail)?)
            .subject(email.subject)
            .multipart(email_parts)
            .map_err(|_| MailerError::BuildEmailError)
    }

    /// Sends a mail.
    ///
    /// # Arguments
    ///
    /// * `email` - The email to send
    ///
    /// # Returns
    ///
    /// Returns `()` if the email is sent.
    ///
    /// # Errors
    ///
    /// Returns `MailerError::SmtpSendError` if the email cannot be sent.
    async fn send_mail(&self, email: Message) -> Result<(), MailerError> {
        match self.mailer.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error sending email: {e}");
                Err(MailerError::SmtpSendError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_transport() -> AsyncSmtpTransport<Tokio1Executor> {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("localhost").port(25).build()
    }

    #[tokio::test]
    async fn test_create_mail_success() {
        let mailer = Mailer {
            mailer: setup_mock_transport(),
            origin_email: "test@test.com".to_string(),
        };

        let email = Email {
            to: "recipient@test.com".to_string(),
            subject: "Test Subject".to_string(),
            html_body: "<h1>Test Body</h1>".to_string(),
            attachment: None,
        };

        let result = mailer.create_mail(email);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_mail_with_attachment() {
        let mailer = Mailer {
            mailer: AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("localhost").port(25).build(),
            origin_email: "test@test.com".to_string(),
        };

        let email = Email {
            to: "recipient@test.com".to_string(),
            subject: "Test Subject".to_string(),
            html_body: "<h1>Test Body</h1>".to_string(),
            attachment: Some(vec![1, 2, 3, 4]), // Mock PDF data
        };

        let result = mailer.create_mail(email);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_mail_invalid_origin_email() {
        let mailer = Mailer {
            mailer: AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("localhost").port(25).build(),
            origin_email: "invalid-email".to_string(),
        };

        let email = Email {
            to: "recipient@test.com".to_string(),
            subject: "Test Subject".to_string(),
            html_body: "<h1>Test Body</h1>".to_string(),
            attachment: None,
        };

        let result = mailer.create_mail(email);
        assert!(matches!(result, Err(MailerError::InvalidOriginEmail)));
    }

    #[tokio::test]
    async fn test_create_mail_invalid_recipient_email() {
        let mailer = Mailer {
            mailer: AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("localhost").port(25).build(),
            origin_email: "test@test.com".to_string(),
        };

        let email = Email {
            to: "invalid-email".to_string(),
            subject: "Test Subject".to_string(),
            html_body: "<h1>Test Body</h1>".to_string(),
            attachment: None,
        };

        let result = mailer.create_mail(email);
        assert!(matches!(result, Err(MailerError::InvalidRecipientEmail)));
    }

    #[tokio::test]
    async fn test_send_mail_error() {
        let mailer = Mailer {
            mailer: setup_mock_transport(),
            origin_email: "test@test.com".to_string(),
        };

        let message = Message::builder()
            .from("test@test.com".parse().unwrap())
            .to("recipient@test.com".parse().unwrap())
            .subject("Test")
            .body("Test body".to_string())
            .unwrap();

        let result = mailer.send_mail(message).await;
        assert!(matches!(result, Err(MailerError::SmtpSendError)));
    }
}
