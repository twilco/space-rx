extern crate space_rx;

use space_rx::SortDir;
use space_rx::v2_api::models::info::*;

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
//
//#[test]
//fn all_rockets() {
//    assert!(v2_api::all_rockets().is_ok());
//}
//
//#[test]
//fn specific_rocket() {
//    assert!(v2_api::rocket("falcon9").is_ok());
//}
//
//#[test]
//fn all_capsules() {
//    assert!(v2_api::all_capsules().is_ok());
//}
//
//#[test]
//fn specific_capsule() {
//    assert!(v2_api::capsule("dragon1").is_ok());
//}
//
//#[test]
//fn all_launchpads() {
//    assert!(v2_api::all_launchpads().is_ok());
//}
//
//#[test]
//fn specific_launchpad() {
//    assert!(v2_api::launchpad("kwajalein_atoll").is_ok());
//}
//
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