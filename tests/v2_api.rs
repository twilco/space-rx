extern crate space_rx;

use space_rx::*;

#[test]
fn company() {
    assert!(v2_api::company().is_ok());
}

#[test]
fn bunch_of_random_historical_events() {
    assert!(v2_api::historical_events(None, None, None, None).is_ok());
}

#[test]
fn historical_event_for_flight() {
    assert!(v2_api::historical_events(None, None, Some(6), Some(v2_api::SortDir::DESC)).is_ok());
}

#[test]
fn historical_event_for_date_range() {
    assert!(v2_api::historical_events(Some("2017-06-22".to_owned()), Some("2018-06-25".to_owned()), None, Some(v2_api::SortDir::ASC)).is_ok());
}

#[test]
fn all_rockets() {
    assert!(v2_api::all_rockets().is_ok());
}

#[test]
fn specific_rocket() {
    assert!(v2_api::rocket("falcon9").is_ok());
}

#[test]
fn all_capsules() {
    assert!(v2_api::all_capsules().is_ok());
}

#[test]
fn specific_capsule() {
    assert!(v2_api::capsule("dragon1").is_ok());
}

#[test]
fn all_launchpads() {
    assert!(v2_api::all_launchpads().is_ok());
}

#[test]
fn specific_launchpad() {
    assert!(v2_api::launchpad("kwajalein_atoll").is_ok());
}

#[test]
fn latest_launch() {
    assert!(v2_api::latest_launch().is_ok());
}

#[test]
fn next_launch() {
    assert!(v2_api::next_launch().is_ok());
}

#[test]
fn past_launches() {
    assert!(v2_api::past_launches().is_ok());
}

#[test]
fn upcoming_launches() {
    assert!(v2_api::upcoming_launches().is_ok());
}

#[test]
fn all_launches() {
    assert!(v2_api::all_launchpads().is_ok());
}