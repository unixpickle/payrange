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
    pub updated: Option<u64>,

    pub version: Option<i32>,
    pub firmware: Option<i32>,
    #[serde(rename = "modemFWinstalled")]
    pub modem_fw_installed: Option<i32>,
    #[serde(rename = "fwBaseVersion")]
    pub fw_base_version: Option<i32>,

    pub location: Option<Location>,
    #[serde(rename = "geoAddress")]
    pub geo_address: Option<Location>,
    pub geocode: Option<Geocode>,

    pub operator: Option<String>,
    pub claimed: Option<u64>,
    #[serde(rename = "claimedBy")]
    pub claimed_by: Option<String>,
    #[serde(rename = "operatorContact")]
    pub operator_contact: Option<OperatorContact>,

    pub machine: Option<String>,
    #[serde(rename = "machinePosition")]
    pub machine_position: Option<i32>,
    pub machine_id: Option<String>,
    pub spid: Option<Vec<String>>,

    pub category: Option<Category>,
    #[serde(rename = "deviceType")]
    pub device_type: String,

    #[serde(rename = "cashPerPulse")]
    pub cash_per_pulse: Option<i32>,
    #[serde(rename = "pulseAmount")]
    pub pulse_amount: Option<i32>,
    #[serde(rename = "pulseOnTime")]
    pub pulse_on_time: Option<i32>,
    #[serde(rename = "pulseOffTime")]
    pub pulse_off_time: Option<i32>,
    #[serde(rename = "pulseInhibThresh")]
    pub pulse_inhib_thresh: Option<i32>,
    #[serde(rename = "pulseMaxWidth")]
    pub pulse_max_width: Option<i32>,

    #[serde(rename = "type")]
    pub currency_type: Option<String>,
    #[serde(rename = "creditOptions")]
    pub credit_options: Option<Vec<CreditOption>>,
    pub surcharge_message: Option<String>,
    pub image: Option<String>,
    pub thumbnail: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub locality: Option<String>,
    pub region: Option<String>,
    pub post_code: Option<String>,
    pub country: Option<String>,
    pub address: Option<Vec<String>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub service_location: Option<String>,
    pub top_level: Option<String>,
    pub sub_level: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditOption {
    pub pulse_count: Option<i32>,
    pub default: Option<bool>,
    pub price: i32,
    pub short_description: String,
    pub display_price: String,
    pub md5_hash: String,
    pub description: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorContact {
    pub company: String,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub operator_logo_url: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Geocode {
    #[serde(rename = "type")]
    pub code_type: String,
    pub coordinates: Vec<f64>
}
