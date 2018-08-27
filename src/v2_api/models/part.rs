#[derive(Debug, Deserialize)]
pub struct CorePart {
    asds_attempt: bool,
    asds_landings: u16,
    block: Option<u16>,
    core_serial: String,
    details: Option<String>,
    missions: Vec<String>,
    original_launch: Option<String>,
    original_launch_unix: Option<u64>,
    rtls_attempt: bool,
    rtls_landings: u16,
    status: String
}

#[derive(Debug, Deserialize)]
pub struct CapsulePart {
    capsule_serial: String,
    capsule_id: String,
    details: Option<String>,
    landings: u16,
    missions: Vec<String>,
    original_launch: Option<String>,
    original_launch_unix: Option<u64>,
    #[serde(rename="type")]
    part_type: String,
    status: String
}