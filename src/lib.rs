#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate url;

pub mod models;
use ::models::*;

use std::fmt;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use reqwest::Client;
use url::Url;

fn send_request<T>(endpoint: &str, params: Option<HashMap<&str, String>>) -> Result<T, failure::Error>
    where T: DeserializeOwned
{
    let base = &("https://api.spacexdata.com/v2/".to_owned() + endpoint);
    let url = match params {
        Some(map) => Url::parse_with_params(base, map),
        None => Url::parse(base)
    };
    let client = Client::new();
    let mut request = client.get(url.unwrap().as_str());
    let mut response = request.send()?;
    Ok(response.json()?)
}

#[derive(Debug)]
pub enum SortDir {
    ASC,
    DESC
}

impl fmt::Display for SortDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn company_info() -> Result<CompanyInfo, failure::Error>
{
    send_request("info", None)
}

pub fn history(start_date: Option<String>, end_date: Option<String>, flight_number: Option<u32>, sort_dir: Option<SortDir>) -> Result<Vec<HistoricalEvent>, failure::Error>
{
    let mut params = HashMap::new();

    if let Some(start) = start_date {
        params.insert("start", start);
    }

    if let Some(end) = end_date {
        params.insert("end", end);
    }

    if let Some(flight_num) = flight_number {
        params.insert("flight_number", flight_num.to_string());
    }

    if let Some(dir) = sort_dir {
        params.insert("order", dir.to_string());
    }

    send_request("info/history", Some(params))
}

pub fn all_rocket_info() -> Result<Vec<Rocket>, failure::Error> {
    send_request("rockets", None)
}

pub fn rocket_info(rocket_id: &str) -> Result<Rocket, failure::Error> {
    if rocket_id.is_empty() {
        bail!("cannot call rocket_info with an empty &str");
    }

    let endpoint = "rockets/".to_owned() + rocket_id;
    send_request(endpoint.as_str(), None)
}