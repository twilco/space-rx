extern crate space_rx;

use space_rx::v2_api::requests::info::*;

#[test]
fn info_request() {
    assert!(space_rx::send(&InfoRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn info_roadster_request() {
    assert!(space_rx::send(&RoadsterInfoRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn bunch_of_random_historical_events() {
    assert!(space_rx::send(&CompanyHistoryRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn historical_event_for_flight() {
    assert!(space_rx::send(&CompanyHistoryRequestBuilder::default().flight_number(6).build().unwrap()).is_ok());
}

#[test]
fn historical_event_for_date_range() {
    assert!(space_rx::send(&CompanyHistoryRequestBuilder::default().start("2017-06-22").end("2018-06-25").build().unwrap()).is_ok());
}