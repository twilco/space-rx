extern crate space_rx;

use space_rx::v2_api::requests::rocket::*;

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