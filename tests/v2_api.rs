extern crate space_rx;

use space_rx::SortDir;
use space_rx::v2_api::requests::info::*;
use space_rx::v2_api::requests::rocket::*;
use space_rx::v2_api::requests::capsule::*;
use space_rx::v2_api::requests::launchpad::*;

#[test]
fn info_request() {
    assert!(space_rx::send(InfoRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn info_roadster_request() {
    assert!(space_rx::send(InfoRoadsterRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn bunch_of_random_historical_events() {
    assert!(space_rx::send(InfoHistoryRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn historical_event_for_flight() {
    assert!(space_rx::send(InfoHistoryRequestBuilder::default().flight_number(6).build().unwrap()).is_ok());
}

#[test]
fn historical_event_for_date_range() {
    assert!(space_rx::send(InfoHistoryRequestBuilder::default().start("2017-06-22").end("2018-06-25").build().unwrap()).is_ok());
}

#[test]
fn all_rockets() {
    assert!(space_rx::send(AllRocketsRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn specific_rocket() {
    assert!(space_rx::send(RocketRequestBuilder::default().rocket_id("falcon9").build().unwrap()).is_ok());
}

#[test]
fn specific_rocket_request_builder_fails_with_no_rocket_id() {
    assert!(space_rx::send(RocketRequestBuilder::default().build().unwrap()).is_err());
}

#[test]
fn all_capsules() {
    assert!(space_rx::send(AllCapsulesRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn specific_capsule() {
    assert!(space_rx::send(CapsuleRequestBuilder::default().capsule_id("dragon1").build().unwrap()).is_ok());
}

#[test]
fn specific_capsule_request_builder_fails_with_no_capsule_id() {
    assert!(space_rx::send(CapsuleRequestBuilder::default().build().unwrap()).is_err());
}

#[test]
fn all_launchpads() {
    assert!(space_rx::send(AllLaunchpadsRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn specific_launchpad() {
    assert!(space_rx::send(LaunchpadRequestBuilder::default().launchpad_id("kwajalein_atoll").build().unwrap()).is_ok());
}

#[test]
fn specific_launchpad_request_builder_fails_with_no_launchpad_id() {
    assert!(space_rx::send(LaunchpadRequestBuilder::default().build().unwrap()).is_err());
}

//#[test]
//fn latest_launch() {
//    assert!(v2_api::latest_launch().is_ok());
//}
//
//#[test]
//fn next_launch() {
//    assert!(v2_api::next_launch().is_ok());
//}
//
//#[test]
//fn past_launches() {
//    assert!(v2_api::past_launches().is_ok());
//}
//
//#[test]
//fn upcoming_launches() {
//    assert!(v2_api::upcoming_launches().is_ok());
//}
//
//#[test]
//fn all_launches() {
//    assert!(v2_api::all_launchpads().is_ok());
//}