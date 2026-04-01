use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::header::ContentType,
    transport::smtp::{Error, authentication::Credentials},
};

use crate::{
    config::{EmailSettings, Settings},
    domain::infrastructure::email::EmailSender,
    infrastructure::email::templates::EmailTemplates,
};

pub struct SmtpService {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    settings: EmailSettings,
}

impl SmtpService {
    pub fn new(settings: Settings) -> Result<Self, Error> {
        let email_set = settings.email;
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&email_set.smtp_host)?
            .credentials(Credentials::new(
                email_set.username.clone(),
                email_set.password.clone(),
            ))
            .build();
        Ok(Self {
            transport,
            settings: email_set,
        })
    }

    async fn send_verification_email(
        &self,
        to_email: &str,
        name: &str,
        code: &str,
    ) -> anyhow::Result<()> {
        let html_body =
            EmailTemplates::get_verification_html(name, code, self.settings.code_expire_min);
        let msg = Message::builder()
            .from(format!("Cache Flow <{}>", self.settings.from).parse()?)
            .to(format!("{name} <{to_email}>").parse()?)
            .header(ContentType::TEXT_HTML)
            .subject("Cache Flow Registration")
            .body(html_body)?;
        self.transport.send(msg).await?;
        Ok(())
    }
}

#[async_trait]
impl EmailSender for SmtpService {
    async fn send_verification_code(
        &self,
        to_email: &str,
        name: &str,
        code: &str,
    ) -> anyhow::Result<()> {
        self.send_verification_email(to_email, name, code).await
    }
}
