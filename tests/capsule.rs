extern crate space_rx;

use space_rx::v2_api::requests::capsule::*;

#[test]
fn all_capsules() {
    assert!(space_rx::send(&AllCapsulesRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn specific_capsule() {
    assert!(space_rx::send(&CapsuleRequestBuilder::default().capsule_id("dragon1").build().unwrap()).is_ok());
}

#[test]
fn specific_capsule_request_builder_fails_with_no_capsule_id() {
    assert!(space_rx::send(&CapsuleRequestBuilder::default().build().unwrap()).is_err());
}
