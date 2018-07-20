#[derive(Debug, Deserialize)]
pub struct CompanyInfo {
    name: String,
    founder: String,
    founded: u16,
    employees: u32,
    vehicles: u32,
    launch_sites: u16,
    test_sites: u16,
    ceo: String,
    cto: String,
    coo: String,
    cto_propulsion: String,
    valuation: u64,
    headquarters: Headquarters,
    summary: String
}

#[derive(Debug, Deserialize)]
struct Headquarters {
    address: String,
    city: String,
    state: String
}

#[derive(Debug, Deserialize)]
pub struct HistoricalEvent {
    title: String,
    event_date_utc: String,
    event_date_unix: u64,
    flight_number: Option<u32>,
    details: Option<String>,
    links: EventLinks
}

#[derive(Debug, Deserialize)]
struct EventLinks {
    reddit: Option<String>,
    article: Option<String>,
    wikipedia: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Rocket {
    id: String,
    name: String,
    #[serde(rename="type")]
    rocket_type: String,
    active: bool,
    stages: u16,
    boosters: u16,
    cost_per_launch: u64,
    success_rate_pct: u8,
    first_flight: String,
    country: String,
    company: String,
    height: Length,
    diameter: Length,
    mass: Weight,
    payload_weights: Vec<PayloadWeight>,
    first_stage: FirstStage,
    second_stage: SecondStage,
    engines: Engines,
    landing_legs: LandingLegs,
    description: String
}

#[derive(Debug, Deserialize)]
struct Length {
    meters: Option<f64>,
    feet: Option<f64>
}

#[derive(Debug, Deserialize)]
struct Volume {
    cubic_meters: u16,
    cubic_feet: u16
}

#[derive(Debug, Deserialize)]
struct Weight {
    kg: u32,
    lb: u32
}

#[derive(Debug, Deserialize)]
struct Force {
    #[serde(rename="kN")]
    kn: f64,
    lbf: f64
}

#[derive(Debug, Deserialize)]
struct PayloadWeight {
    id: String,
    name: String,
    kg: u32,
    lb: u32
}

#[derive(Debug, Deserialize)]
struct PayloadVolume {
    cubic_meters: u16,
    cubic_feet: u16
}

#[derive(Debug, Deserialize)]
struct SecondStagePayloads {
    option_1: Option<String>,
    option_2: Option<String>,
    composite_fairing: CompositeFairing
}

#[derive(Debug, Deserialize)]
struct CompositeFairing {
    height: Length,
    diameter: Length
}

#[derive(Debug, Deserialize)]
struct FirstStage {
    reusable: bool,
    engines: u16,
    fuel_amount_tons: f64,
    burn_time_sec: u32,
    thrust_sea_level: Force,
    thrust_vacuum: Force
}

#[derive(Debug, Deserialize)]
struct SecondStage {
    engines: u16,
    fuel_amount_tons: Option<f64>,
    burn_time_sec: u32,
    thrust: Force,
    payloads: SecondStagePayloads
}

#[derive(Debug, Deserialize)]
struct Engines {
    number: u16,
    #[serde(rename="type")]
    engine_type: String,
    version: String,
    layout: Option<String>,
    engine_loss_max: Option<u16>,
    propellant_1: String,
    propellant_2: String,
    thrust_sea_level: Force,
    thrust_vacuum: Force,
    thrust_to_weight: Option<f64>
}

#[derive(Debug, Deserialize)]
struct LandingLegs {
    number: u8,
    material: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Capsule {
    id: String,
    name: String,
    #[serde(rename="type")]
    capsule_type: String,
    active: bool,
    crew_capacity: u16,
    sidewall_angle_deg: i16,
    orbit_duration_yr: u16,
    heat_shield: HeatShield,
    thrusters: Vec<Thruster>,
    launch_payload_mass: Weight,
    launch_payload_vol: Volume,
    return_payload_mass: Weight,
    return_payload_vol: Volume,
    pressurized_capsule: PressurizedCapsule,
    trunk: Trunk,
    height_w_trunk: Length,
    diameter: Length
}

#[derive(Debug, Deserialize)]
struct PressurizedCapsule {
    payload_volume: PayloadVolume
}

#[derive(Debug, Deserialize)]
struct HeatShield {
    material: String,
    size_meters: f64,
    temp_degrees: i16,
    dev_partner: String
}

#[derive(Debug, Deserialize)]
struct Thruster {
    #[serde(rename="type")]
    thruster_type: String,
    amount: u16,
    pods: u8,
    fuel_1: String,
    fuel_2: String,
    thrust: Force
}

#[derive(Debug, Deserialize)]
struct Trunk {
    trunk_volume: TrunkVolume,
    cargo: Cargo
}

#[derive(Debug, Deserialize)]
struct TrunkVolume {
    cubic_meters: u16,
    cubic_feet: u16
}

#[derive(Debug, Deserialize)]
struct Cargo {
    solar_array: u8,
    unpressurized_cargo: bool
}

#[derive(Debug, Deserialize)]
pub struct Launchpad {
    id: String,
    full_name: String,
    status: String,
    location: LaunchpadLocation,
    vehicles_launched: Vec<String>,
    details: String
}

#[derive(Debug, Deserialize)]
struct LaunchpadLocation {
    name: String,
    region: String,
    latitude: f64,
    longitude: f64
}

#[derive(Debug, Deserialize)]
pub struct Launch {
    flight_number: u16,
    mission_name: String,
    launch_year: String,
    launch_date_unix: u64,
    launch_date_utc: String,
    launch_date_local: String,
    rocket: LaunchRocket,
    telemetry: Telemetry,
    reuse: Reuse,
    launch_site: LaunchSite,
    launch_success: bool,
    links: LaunchLinks,
    details: String
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
    mission_patch: String,
    mission_patch_small: String,
    article_link: String,
    wikipedia: String,
    video_link: String
}

#[derive(Debug, Deserialize)]
struct LaunchFirstStage {
    cores: Vec<Core>
}

#[derive(Debug, Deserialize)]
struct LaunchSecondStage {
    block: Option<u16>,
    payloads: Vec<LaunchPayload>
}

#[derive(Debug, Deserialize)]
struct LaunchPayload {
    payload_id: String,
    reused: bool,
    customers: Vec<String>,
    payload_type: String,
    payload_mass_kg: Option<u32>,
    payload_mass_lbs: Option<u32>,
    orbit: String,
    orbit_params: OrbitParams
}

#[derive(Debug, Deserialize)]
struct OrbitParams {
    reference_system: String,
    regime: String,
    longitude: Option<f64>,
    semi_major_axis_km: Option<u32>,
    eccentricity: Option<f64>,
    periapsis_km: Option<u32>,
    apoapsis_km: Option<u32>,
    inclination_deg: Option<f64>,
    period_min: Option<f64>,
    lifespan_years: Option<u16>
}

#[derive(Debug, Deserialize)]
struct Core {
    core_serial: String,
    flight: u16,
    block: Option<u16>,
    reused: bool,
    land_success: Option<bool>,
    landing_type: Option<String>,
    landing_vehicle: Option<String>
}