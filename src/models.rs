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
    links: Links
}

#[derive(Debug, Deserialize)]
struct Links {
    reddit: Option<String>,
    article: Option<String>,
    wikipedia: Option<String>
}