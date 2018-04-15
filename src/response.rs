#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub auth: UserAuth,
    #[serde(rename = "_id")]
    pub id: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAuth {
    pub email: Option<String>,
    pub token: Option<UserAuthToken>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAuthToken {
    #[serde(rename = "tokenString")]
    pub token_string: String,
    pub created: u64,
    pub expiry: u64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeviceResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub created: u64,
    pub updated: u64,
    pub location: Option<Location>,
    #[serde(rename = "machinePosition")]
    pub machine_position: Option<i32>,
    pub operator: Option<String>,
    pub machine: Option<String>,
    pub machine_id: Option<String>,
    pub category: Option<Category>,
    #[serde(rename = "type")]
    pub currency_type: Option<String>,
    pub claimed: Option<u64>,
    #[serde(rename = "claimedBy")]
    pub claimed_by: Option<String>,
    pub spid: Option<Vec<String>>,
    #[serde(rename = "deviceType")]
    pub device_type: String,
    #[serde(rename = "creditOptions")]
    pub credit_options: Option<Vec<CreditOption>>,
    pub image: Option<String>,
    pub thumbnail: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub locality: String,
    pub region: String,
    pub post_code: String,
    pub address: Vec<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub service_location: String,
    pub top_level: String,
    pub sub_level: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditOption {
    pub pulse_count: Option<i32>,
    pub default: bool,
    pub price: i32,
    pub short_description: String,
    pub display_price: String,
    pub md5_hash: String,
    pub description: String
}
