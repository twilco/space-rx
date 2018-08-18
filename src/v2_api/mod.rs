use self::super::*;

use std::collections::HashMap;
use serde::de::DeserializeOwned;
use reqwest::Client;
use url::Url;
use failure;

pub mod models;
pub use self::models::capsule::*;
pub use self::models::common::*;
pub use self::models::info::*;
pub use self::models::launch::*;
pub use self::models::launchpad::*;
pub use self::models::rocket::*;

// TODO: lift out of v2 module - generalize v2 in URL
//fn send_request<T>(endpoint: &str, params: Option<HashMap<&str, String>>) -> Result<T, failure::Error>
//    where T: DeserializeOwned
//{
//    let base = &("https://api.spacexdata.com/v2/".to_owned() + endpoint);
//    let url = match params {
//        Some(map) => Url::parse_with_params(base, map),
//        None => Url::parse(base)
//    };
//    let client = Client::new();
//    let mut request = client.get(url.unwrap().as_str());
//    let mut response = request.send()?;
//    Ok(response.json()?)
//}

//pub fn company(req: InfoRequest) -> Result<CompanyInfo, failure::Error>
//{
//    send_request(req)
//}
//
//pub fn historical_events(start_date: Option<String>, end_date: Option<String>, flight_number: Option<u32>, sort_dir: Option<SortDir>) -> Result<Vec<HistoricalEvent>, failure::Error>
//{
//    let mut params = HashMap::new();
//
//    if let Some(start) = start_date {
//        params.insert("start", start);
//    }
//
//    if let Some(end) = end_date {
//        params.insert("end", end);
//    }
//
//    if let Some(flight_num) = flight_number {
//        params.insert("flight_number", flight_num.to_string());
//    }
//
//    if let Some(dir) = sort_dir {
//        params.insert("order", dir.to_string());
//    }
//
//    send_request("info/history", Some(params))
//}
//
//pub fn all_rockets() -> Result<Vec<Rocket>, failure::Error> {
//    send_request("rockets", None)
//}
//
//pub fn rocket(rocket_id: &str) -> Result<Rocket, failure::Error> {
//    if rocket_id.is_empty() {
//        bail!("cannot call rocket() with an empty &str");
//    }
//
//    let endpoint = "rockets/".to_owned() + rocket_id;
//    send_request(endpoint.as_str(), None)
//}
//
//pub fn all_capsules() -> Result<Vec<Capsule>, failure::Error> {
//    send_request("capsules", None)
//}
//
//pub fn capsule(capsule_id: &str) -> Result<Capsule, failure::Error> {
//    if capsule_id.is_empty() {
//        bail!("cannot call capsule() with an empty capsule id");
//    }
//
//    let endpoint = "capsules/".to_owned() + capsule_id;
//    send_request(endpoint.as_str(), None)
//}
//
//pub fn all_launchpads() -> Result<Vec<Launchpad>, failure::Error> {
//    send_request("launchpads", None)
//}
//
//pub fn launchpad(launchpad_id: &str) -> Result<Launchpad, failure::Error> {
//    if launchpad_id.is_empty() {
//        bail!("cannot call launchpad() with an empty launchpad id");
//    }
//
//    let endpoint = "launchpads/".to_owned() + launchpad_id;
//    send_request(endpoint.as_str(), None)
//}
//
//pub fn latest_launch() -> Result<Launch, failure::Error> {
//    send_request("launches/latest", None)
//}
//
//pub fn next_launch() -> Result<Launch, failure::Error> {
//    send_request("launches/next", None)
//}
//
//pub fn past_launches() -> Result<Vec<Launch>, failure::Error> {
//    send_request("launches", None)
//}
//
//pub fn upcoming_launches() -> Result<Vec<Launch>, failure::Error> {
//    send_request("launches/upcoming", None)
//}
//
//pub fn all_launches() -> Result<Vec<Launch>, failure::Error> {
//    send_request("launches/all", None)
//}
