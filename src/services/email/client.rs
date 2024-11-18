use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SendEmailError {
    #[error("Failed to send email")]
    FailedToSendEmail,
}

#[derive(Clone)]
pub struct EmailClient {
    #[allow(unused)]
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

#[allow(unused)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub html_body: String,
}

impl EmailClient {
    pub fn new(smtp_username: String, smtp_password: String, smtp_host: &str) -> Self {
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
            .unwrap()
            .credentials(credentials)
            .build();

        Self { mailer }
    }

    #[allow(unused)]
    pub fn send_email(&self, email: Email) -> Result<(), SendEmailError> {
        // TODO: Implement email sending
        Err(SendEmailError::FailedToSendEmail)
    }
}
