pub struct EmailTemplates;

impl EmailTemplates {
    pub fn get_verification_html(name: &str, code: &str, expire_min: u16) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <body style="margin: 0; padding: 0; font-family: 'Inter', Helvetica, Arial, sans-serif; background-color: #ffffff;">
                <div style="max-width: 400px; margin: 0 auto; padding: 40px 20px; text-align: center;">
                    <div style="color: #4F39F6; font-weight: 700; font-size: 24px; margin-bottom: 30px;">CacheFlow</div>
                    <h1 style="font-size: 24px; color: #1a1a1a; margin-bottom: 8px;">Подтвердите почту</h1>
                    <p style="color: #666666; font-size: 15px; margin-bottom: 32px;">Привет, {name}! Ваш код подтверждения:</p>

                    <div style="background: #f4f4f9; border-radius: 12px; padding: 20px; border: 2px solid #e2e2e7; margin-bottom: 32px;">
                        <span style="font-size: 32px; font-weight: 700; color: #4F39F6; letter-spacing: 8px;">{code}</span>
                    </div>

                    <p style="color: #999999; font-size: 12px;">Код действителен в течение {expire_min} минут. Если вы не запрашивали его, просто проигнорируйте это письмо.</p>
                </div>
            </body>
            </html>
            "#,
            name = name,
            code = code,
            expire_min = expire_min
        )
    }
}
