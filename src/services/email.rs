use crate::utils::Email;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
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

    /// Sends an email.
    ///
    /// # Errors
    ///
    /// Returns `MailerError::SmtpSendError` if the email cannot be sent.
    pub async fn send_email(&self, email: Email) -> Result<(), MailerError> {
        let send_email_request = Message::builder()
            .from(self.origin_email.parse().map_err(|_| MailerError::InvalidOriginEmail)?)
            .to(email.to.parse().map_err(|_| MailerError::InvalidRecipientEmail)?)
            .subject(email.subject)
            .header(ContentType::TEXT_HTML)
            .body(email.html_body)
            .map_err(|_| MailerError::BuildEmailError)?;

        match self.mailer.send(send_email_request).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error sending email: {e}");
                Err(MailerError::SmtpSendError)
            }
        }
    }
}
