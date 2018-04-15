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
    pub location: Location,
    #[serde(rename = "machinePosition")]
    pub machine_position: i32,
    pub operator: String,
    pub machine: String,
    pub machine_id: String,
    pub category: Category,
    #[serde(rename = "type")]
    pub currency_type: String,
    pub claimed: u64,
    #[serde(rename = "claimedBy")]
    pub claimed_by: String,
    pub spid: Vec<String>,
    #[serde(rename = "deviceType")]
    pub device_type: String,
    #[serde(rename = "creditOptions")]
    pub credit_options: Vec<CreditOption>,
    pub image: String,
    pub thumbnail: String
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
