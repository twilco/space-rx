#[derive(Debug, Deserialize)]
pub struct Launch {
    flight_number: u32,
    mission_name: String,
    launch_year: String,
    launch_date_unix: u64,
    launch_date_utc: String,
    launch_date_local: String,
    rocket: LaunchRocket,
    telemetry: Telemetry,
    reuse: Reuse,
    launch_site: LaunchSite,
    launch_success: Option<bool>,
    links: LaunchLinks,
    details: Option<String>,
    upcoming: bool
}

#[derive(Debug, Deserialize)]
struct LaunchRocket {
    rocket_id: String,
    rocket_name: String,
    rocket_type: String,
    first_stage: LaunchFirstStage,
    second_stage: LaunchSecondStage
}

#[derive(Debug, Deserialize)]
struct LaunchFirstStage {
    cores: Vec<Core>
}

#[derive(Debug, Deserialize)]
struct Core {
    core_serial: Option<String>,
    flight: Option<u16>,
    block: Option<u16>,
    reused: Option<bool>,
    land_success: Option<bool>,
    landing_type: Option<String>,
    landing_vehicle: Option<String>
}

#[derive(Debug, Deserialize)]
struct LaunchSecondStage {
    block: Option<u16>,
    payloads: Vec<LaunchPayload>
}

#[derive(Debug, Deserialize)]
struct LaunchPayload {
    payload_id: String,
    norad_id: Option<Vec<u32>>,
    reused: bool,
    customers: Vec<String>,
    nationality: Option<String>,
    manufacturer: Option<String>,
    payload_type: String,
    payload_mass_kg: Option<f64>,
    payload_mass_lbs: Option<f64>,
    orbit: String,
    orbit_params: OrbitParams
}

#[derive(Debug, Deserialize)]
struct OrbitParams {
    reference_system: String,
    regime: Option<String>,
    longitude: Option<f64>,
    semi_major_axis_km: Option<f64>,
    eccentricity: Option<f64>,
    periapsis_km: Option<f64>,
    apoapsis_km: Option<f64>,
    inclination_deg: Option<f64>,
    period_min: Option<f64>,
    lifespan_years: Option<u32>
}

#[derive(Debug, Deserialize)]
struct Telemetry {
    flight_club: Option<String>
}

#[derive(Debug, Deserialize)]
struct Reuse {
    core: bool,
    side_core1: bool,
    side_core2: bool,
    fairings: bool,
    capsule: bool
}

#[derive(Debug, Deserialize)]
struct LaunchSite {
    site_id: String,
    site_name: String,
    site_name_long: String
}

#[derive(Debug, Deserialize)]
struct LaunchLinks {
    mission_patch: Option<String>,
    mission_patch_small: Option<String>,
    reddit_campaign: Option<String>,
    reddit_launch: Option<String>,
    reddit_media: Option<String>,
    presskit: Option<String>,
    article_link: Option<String>,
    wikipedia: Option<String>,
    video_link: Option<String>
}