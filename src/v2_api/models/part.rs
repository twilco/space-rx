#[derive(Debug, Deserialize)]
pub struct CorePart {
    pub asds_attempt: bool,
    pub asds_landings: u16,
    pub block: Option<u16>,
    pub core_serial: String,
    pub details: Option<String>,
    pub missions: Vec<String>,
    pub original_launch: Option<String>,
    pub original_launch_unix: Option<u64>,
    pub rtls_attempt: bool,
    pub rtls_landings: u16,
    pub status: String
}

#[derive(Debug, Deserialize)]
pub struct CapsulePart {
    pub capsule_serial: String,
    pub capsule_id: String,
    pub details: Option<String>,
    pub landings: u16,
    pub missions: Vec<String>,
    pub original_launch: Option<String>,
    pub original_launch_unix: Option<u64>,
    #[serde(rename="type")]
    pub part_type: String,
    pub status: String
}