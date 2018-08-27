use std::collections::HashMap;

use ::ApiRequest;
use ::SortDir;
use v2_api::models::info::*;

// TODO: Rename these requests to not match the request endpoint exactly, but more of a semantic name.  Or change semantic names to match this pattern of exact endpoint matching

/// Retrieves basic information about SpaceX.
///
/// Endpoint is v2/info.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct InfoRequest {}

impl ApiRequest for InfoRequest {
    type Output = CompanyInfo;

    fn endpoint(&self) -> String {
        "v2/info".to_owned()
    }
}

/// Retrieves orbital and other more general information about the Roadster launched into Space, driven
/// by none other than Starman.
///
/// According to the API documentation, this data is updated every 5 minutes.
///
/// Endpoint is v2/info/roadster.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct InfoRoadsterRequest {}

impl ApiRequest for InfoRoadsterRequest {
    type Output = RoadsterInfo;

    fn endpoint(&self) -> String {
        "v2/info/roadster".to_owned()
    }
}

/// Retrieves company info and milestones.
///
/// Endpoint is v2/info/history.
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

        if let Some(start) = self.start {
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
