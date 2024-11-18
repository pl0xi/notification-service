use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::time::Duration;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum SendEmailError {
    #[error("Failed to send email")]
    FailedToSendEmail,

    #[error("Failed to parse origin email")]
    FailedToParseOriginEmail,

    #[error("Failed to parse recipient email")]
    FailedToParseRecipientEmail,
}

#[derive(Clone)]
pub struct EmailClient {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    origin_email: String,
}

pub struct Email {
    pub to: String,
    pub subject: String,
    pub html_body: String,
}

impl EmailClient {
    pub fn new(smtp_username: String, smtp_password: String, smtp_host: &str, origin_email: String) -> Self {
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
            .unwrap()
            .credentials(credentials)
            .timeout(Some(Duration::from_secs(10)))
            .build();

        Self { mailer, origin_email }
    }

    pub async fn send_email(&self, email: Email) -> Result<(), SendEmailError> {
        let send_email_request = Message::builder()
            .from(self.origin_email.parse().map_err(|_| SendEmailError::FailedToParseOriginEmail)?)
            .to(email.to.parse().map_err(|_| SendEmailError::FailedToParseRecipientEmail)?)
            .subject(email.subject)
            .header(ContentType::TEXT_HTML)
            .body(email.html_body)
            .map_err(|_| SendEmailError::FailedToSendEmail)?;

        match self.mailer.send(send_email_request).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error sending email: {:?}", e);
                Err(SendEmailError::FailedToSendEmail)
            }
        }
    }
}
