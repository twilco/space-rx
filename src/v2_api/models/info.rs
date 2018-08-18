use std::collections::HashMap;

use ::ApiRequest;
use ::SortDir;

/// Retrieves basic information about SpaceX.
///
/// Endpoint is v2/info
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct InfoRequest {}

impl ApiRequest for InfoRequest {
    type Output = CompanyInfo;

    fn endpoint(&self) -> String {
        "v2/info".to_owned()
    }
}

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

/// Retrieves orbital and other more general information about the Roadster launched into Space, driven
/// by none other than Starman.
///
/// According to the API documentation, this data is updated every 5 minutes.
///
/// Endpoint is v2/info/roadster
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct InfoRoadsterRequest {}

impl ApiRequest for InfoRoadsterRequest {
    type Output = RoadsterInfo;

    fn endpoint(&self) -> String {
        "v2/info/roadster".to_owned()
    }
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

/// Retrieves company info and milestones.
///
/// Endpoint is v2/info/history
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct InfoHistoryRequest<'a> {
    order: Option<SortDir>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    flight_number: Option<u32>
}

impl<'a> ApiRequest for InfoHistoryRequest<'a> {
    type Output = Vec<HistoricalEvent>;

    fn endpoint(&self) -> String {
        "v2/info/history".to_owned()
    }

    fn params(&self) -> Option<HashMap<&str, String>> {
        let mut params = HashMap::new();

        if let Some(order) = &self.order {
            params.insert("order", order.to_string());
        }

        if let Some(start) = &self.start {
            params.insert("start", start.to_string());
        }

        if let Some(end) = &self.end {
            params.insert("end", end.to_string());
        }

        if let Some(flight_number) = &self.flight_number {
            params.insert("flight_number", flight_number.to_string());
        }

        Some(params)
    }
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