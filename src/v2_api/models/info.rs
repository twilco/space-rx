#[derive(Debug, Deserialize)]
pub struct CompanyInfo {
    ceo: String,
    coo: String,
    cto: String,
    cto_propulsion: String,
    employees: u32,
    founded: u16,
    founder: String,
    headquarters: Headquarters,
    launch_sites: u16,
    name: String,
    summary: String,
    test_sites: u16,
    valuation: u64,
    vehicles: u32
}

#[derive(Debug, Deserialize)]
struct Headquarters {
    address: String,
    city: String,
    state: String
}

#[derive(Debug, Deserialize)]
pub struct RoadsterInfo {
    apoapsis_au: f64,
    details: String,
    earth_distance_km: f64,
    earth_distance_mi: f64,
    eccentricity: f64,
    epoch_jd: f64,
    inclination: f64,
    launch_date_unix: u64,
    launch_date_utc: String,
    launch_mass_kg: u16,
    launch_mass_lbs: u16,
    longitude: f64,
    mars_distance_km: f64,
    mars_distance_mi: f64,
    name: String,
    norad_id: u16,
    orbit_type: String,
    periapsis_arg: f64,
    periapsis_au: f64,
    period_days: f64,
    semi_major_axis_au: f64,
    speed_kph: f64,
    speed_mph: f64,
    wikipedia: String
}

#[derive(Debug, Deserialize)]
pub struct HistoricalEvent {
    details: Option<String>,
    event_date_utc: String,
    event_date_unix: u64,
    flight_number: Option<u32>,
    links: EventLinks,
    title: String
}

#[derive(Debug, Deserialize)]
struct EventLinks {
    article: Option<String>,
    reddit: Option<String>,
    wikipedia: Option<String>
}