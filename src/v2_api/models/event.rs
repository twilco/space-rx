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