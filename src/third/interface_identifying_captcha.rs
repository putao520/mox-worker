pub trait IdentifyingCaptcha {
    async fn identifying (&self, captcha: &str) -> anyhow::Result<String>;
}