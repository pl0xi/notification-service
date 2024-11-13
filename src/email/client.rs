use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};

#[derive(Clone)]
pub struct EmailClient {
    #[allow(unused)]
    mailer: AsyncSmtpTransport<Tokio1Executor>,
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
}
