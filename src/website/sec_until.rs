use base64::Engine;
use base64::engine::general_purpose;
pub fn encode_email(email: &str) -> String {
    general_purpose::STANDARD.encode(email)
}