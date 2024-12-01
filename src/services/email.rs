use crate::utils::Email;
use lettre::{
    message::{header::ContentType, Attachment, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::time::Duration;
use thiserror::Error;

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

#[derive(Clone)]
pub struct Mailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    origin_email: String,
}

impl Mailer {
    /// Creates a new mailer.
    ///
    /// # Panics
    ///
    /// Panics if the mailer cannot be created.
    ///
    /// # Errors
    #[must_use]
    pub fn new(smtp_username: String, smtp_password: String, smtp_host: &str, origin_email: String) -> Self {
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
            .unwrap()
            .credentials(credentials)
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
    pub fn create_mail(&self, email: Email) -> Result<Message, MailerError> {
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
    pub async fn send_mail(&self, email: Message) -> Result<(), MailerError> {
        match self.mailer.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error sending email: {e}");
                Err(MailerError::SmtpSendError)
            }
        }
    }
}
