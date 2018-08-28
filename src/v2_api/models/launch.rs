#[derive(Debug, Deserialize)]
pub struct Launch {
    pub details: Option<String>,
    pub flight_number: u32,
    pub launch_date_local: String,
    pub launch_date_unix: u64,
    pub launch_date_utc: String,
    pub launch_site: LaunchSite,
    pub launch_success: Option<bool>,
    pub launch_year: String,
    pub links: LaunchLinks,
    pub mission_name: String,
    pub reuse: Reuse,
    pub rocket: LaunchRocket,
    pub telemetry: Telemetry,
    pub upcoming: bool
}

#[derive(Debug, Deserialize)]
pub struct LaunchRocket {
    pub first_stage: LaunchFirstStage,
    pub rocket_id: String,
    pub rocket_name: String,
    pub rocket_type: String,
    pub second_stage: LaunchSecondStage
}

#[derive(Debug, Deserialize)]
pub struct LaunchFirstStage {
    pub cores: Vec<Core>
}

#[derive(Debug, Deserialize)]
pub struct Core {
    pub block: Option<u16>,
    pub core_serial: Option<String>,
    pub flight: Option<u16>,
    pub land_success: Option<bool>,
    pub landing_type: Option<String>,
    pub landing_vehicle: Option<String>,
    pub reused: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct LaunchSecondStage {
    pub block: Option<u16>,
    pub payloads: Vec<LaunchPayload>
}

#[derive(Debug, Deserialize)]
pub struct LaunchPayload {
    pub customers: Vec<String>,
    pub manufacturer: Option<String>,
    pub nationality: Option<String>,
    pub norad_id: Option<Vec<u32>>,
    pub orbit: String,
    pub orbit_params: OrbitParams,
    pub payload_id: String,
    pub payload_mass_kg: Option<f64>,
    pub payload_mass_lbs: Option<f64>,
    pub payload_type: String,
    pub reused: bool
}

#[derive(Debug, Deserialize)]
pub struct OrbitParams {
    pub apoapsis_km: Option<f64>,
    pub eccentricity: Option<f64>,
    pub inclination_deg: Option<f64>,
    pub lifespan_years: Option<u32>,
    pub longitude: Option<f64>,
    pub periapsis_km: Option<f64>,
    pub period_min: Option<f64>,
    pub reference_system: String,
    pub regime: Option<String>,
    pub semi_major_axis_km: Option<f64>
}

#[derive(Debug, Deserialize)]
pub struct Telemetry {
    pub flight_club: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Reuse {
    pub capsule: bool,
    pub core: bool,
    pub fairings: bool,
    pub side_core1: bool,
    pub side_core2: bool
}

#[derive(Debug, Deserialize)]
pub struct LaunchSite {
    pub site_id: String,
    pub site_name: String,
    pub site_name_long: String
}

#[derive(Debug, Deserialize)]
pub struct LaunchLinks {
    pub article_link: Option<String>,
    pub mission_patch: Option<String>,
    pub mission_patch_small: Option<String>,
    pub presskit: Option<String>,
    pub reddit_campaign: Option<String>,
    pub reddit_launch: Option<String>,
    pub reddit_media: Option<String>,
    pub video_link: Option<String>,
    pub wikipedia: Option<String>
}