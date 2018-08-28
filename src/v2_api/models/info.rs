#[derive(Debug, Deserialize)]
pub struct CompanyInfo {
    pub ceo: String,
    pub coo: String,
    pub cto: String,
    pub cto_propulsion: String,
    pub employees: u32,
    pub founded: u16,
    pub founder: String,
    pub headquarters: Headquarters,
    pub launch_sites: u16,
    pub name: String,
    pub summary: String,
    pub test_sites: u16,
    pub valuation: u64,
    pub vehicles: u32
}

#[derive(Debug, Deserialize)]
pub struct Headquarters {
    pub address: String,
    pub city: String,
    pub state: String
}

#[derive(Debug, Deserialize)]
pub struct RoadsterInfo {
    pub apoapsis_au: f64,
    pub details: String,
    pub earth_distance_km: f64,
    pub earth_distance_mi: f64,
    pub eccentricity: f64,
    pub epoch_jd: f64,
    pub inclination: f64,
    pub launch_date_unix: u64,
    pub launch_date_utc: String,
    pub launch_mass_kg: u16,
    pub launch_mass_lbs: u16,
    pub longitude: f64,
    pub mars_distance_km: f64,
    pub mars_distance_mi: f64,
    pub name: String,
    pub norad_id: u16,
    pub orbit_type: String,
    pub periapsis_arg: f64,
    pub periapsis_au: f64,
    pub period_days: f64,
    pub semi_major_axis_au: f64,
    pub speed_kph: f64,
    pub speed_mph: f64,
    pub wikipedia: String
}

#[derive(Debug, Deserialize)]
pub struct HistoricalEvent {
    pub details: Option<String>,
    pub event_date_utc: String,
    pub event_date_unix: u64,
    pub flight_number: Option<u32>,
    pub links: EventLinks,
    pub title: String
}

#[derive(Debug, Deserialize)]
pub struct EventLinks {
    pub article: Option<String>,
    pub reddit: Option<String>,
    pub wikipedia: Option<String>
}