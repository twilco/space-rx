#[derive(Debug, Deserialize)]
pub struct Launch {
    details: Option<String>,
    flight_number: u32,
    launch_date_local: String,
    launch_date_unix: u64,
    launch_date_utc: String,
    launch_site: LaunchSite,
    launch_success: Option<bool>,
    launch_year: String,
    links: LaunchLinks,
    mission_name: String,
    reuse: Reuse,
    rocket: LaunchRocket,
    telemetry: Telemetry,
    upcoming: bool
}

#[derive(Debug, Deserialize)]
struct LaunchRocket {
    first_stage: LaunchFirstStage,
    rocket_id: String,
    rocket_name: String,
    rocket_type: String,
    second_stage: LaunchSecondStage
}

#[derive(Debug, Deserialize)]
struct LaunchFirstStage {
    cores: Vec<Core>
}

#[derive(Debug, Deserialize)]
struct Core {
    block: Option<u16>,
    core_serial: Option<String>,
    flight: Option<u16>,
    land_success: Option<bool>,
    landing_type: Option<String>,
    landing_vehicle: Option<String>,
    reused: Option<bool>
}

#[derive(Debug, Deserialize)]
struct LaunchSecondStage {
    block: Option<u16>,
    payloads: Vec<LaunchPayload>
}

#[derive(Debug, Deserialize)]
struct LaunchPayload {
    customers: Vec<String>,
    manufacturer: Option<String>,
    nationality: Option<String>,
    norad_id: Option<Vec<u32>>,
    orbit: String,
    orbit_params: OrbitParams,
    payload_id: String,
    payload_mass_kg: Option<f64>,
    payload_mass_lbs: Option<f64>,
    payload_type: String,
    reused: bool
}

#[derive(Debug, Deserialize)]
struct OrbitParams {
    apoapsis_km: Option<f64>,
    eccentricity: Option<f64>,
    inclination_deg: Option<f64>,
    lifespan_years: Option<u32>,
    longitude: Option<f64>,
    periapsis_km: Option<f64>,
    period_min: Option<f64>,
    reference_system: String,
    regime: Option<String>,
    semi_major_axis_km: Option<f64>
}

#[derive(Debug, Deserialize)]
struct Telemetry {
    flight_club: Option<String>
}

#[derive(Debug, Deserialize)]
struct Reuse {
    capsule: bool,
    core: bool,
    fairings: bool,
    side_core1: bool,
    side_core2: bool
}

#[derive(Debug, Deserialize)]
struct LaunchSite {
    site_id: String,
    site_name: String,
    site_name_long: String
}

#[derive(Debug, Deserialize)]
struct LaunchLinks {
    article_link: Option<String>,
    mission_patch: Option<String>,
    mission_patch_small: Option<String>,
    presskit: Option<String>,
    reddit_campaign: Option<String>,
    reddit_launch: Option<String>,
    reddit_media: Option<String>,
    video_link: Option<String>,
    wikipedia: Option<String>
}