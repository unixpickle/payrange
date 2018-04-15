#[derive(Clone, Debug, Serialize)]
pub struct AuthRequest {
    pub auth: AuthRequestBody
}

impl AuthRequest {
    pub fn new_email(email: String, password: String) -> AuthRequest {
        AuthRequest{auth: AuthRequestBody::Email{email: [email, password]}}
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum AuthRequestBody {
    Email{email: [String; 2]}
}
