#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;
extern crate url;

use std::fmt;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use reqwest::Client;
use url::Url;

pub mod v2_api;

pub trait ApiRequest {
    type Output: DeserializeOwned;

    fn endpoint(&self) -> String;

    fn params(&self) -> Option<HashMap<&str, String>> {
        None
    }
}

pub fn send<T: ApiRequest>(request: &T) -> Result<T::Output, failure::Error> {
    let base = &("https://api.spacexdata.com/".to_owned() + &request.endpoint());
    let url = match request.params() {
        Some(map) => Url::parse_with_params(base, map)?,
        None => Url::parse(base)?
    };

    let client = Client::new();
    let mut request = client.get(url.as_str());
    let mut response = request.send()?;
    Ok(response.json()?)
}

#[derive(Clone, Debug)]
pub enum SortDir {
    ASC,
    DESC
}

impl fmt::Display for SortDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

