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

trait ApiRequest {
    type Output: DeserializeOwned;

    fn endpoint(&self) -> String;

    fn params(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn send(&self) -> Result<Self::Output, failure::Error> {
        let base = &("https://api.spacexdata.com/".to_owned() + &Self::endpoint(&self));
        let url = match Self::params(&self) {
            Some(map) => Url::parse_with_params(base, map),
            None => Url::parse(base)
        };

        let client = Client::new();
        let mut request = client.get(url.unwrap().as_str());
        let mut response = request.send()?;
        Ok(response.json()?)
    }
}

#[derive(Debug)]
pub enum SortDir {
    ASC,
    DESC
}

impl fmt::Display for SortDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

