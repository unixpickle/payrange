#[derive(Clone, Debug, Deserialize)]
pub struct UserResponse {
    pub auth: UserAuth,
    #[serde(rename = "_id")]
    pub id: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserAuth {
    pub email: Option<String>,
    pub token: Option<UserAuthToken>
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserAuthToken {
    #[serde(rename = "tokenString")]
    pub token_string: String,
    pub created: u64,
    pub expiry: u64
}
