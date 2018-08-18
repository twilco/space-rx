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
pub struct RoadsterInfo {
    name: String,
    launch_date_utc: String,
    launch_date_unix: u64,
    launch_mass_kg: u16,
    launch_mass_lbs: u16,
    norad_id: u16,
    epoch_jd: String,
    orbit_type: String,
    apoapsis_au: f64,
    periapsis_au: f64,
    semi_major_axis_au: f64,
    eccentricity: f64,
    inclination: f64,
    longitude: f64,
    periapsis_arg: f64,
    period_days: f64,
    speed_kph: f64,
    speed_mph: f64,
    earth_distance_km: f64,
    earth_distance_mi: f64,
    mars_distance_km: f64,
    mars_distance_mi: f64,
    wikipedia: String,
    details: String
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