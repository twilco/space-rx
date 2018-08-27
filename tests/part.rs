extern crate space_rx;

use space_rx::v2_api::requests::part::*;

#[test]
fn all_capsule_parts() {
    assert!(space_rx::send(&AllCapsulePartsRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn all_capsule_parts_with_params() {
    assert!(space_rx::send(&AllCapsulePartsRequestBuilder::default().landings(2).build().unwrap()).is_ok());
}

#[test]
fn parts_for_specific_capsule() {
    assert!(space_rx::send(&CapsulePartRequestBuilder::default().capsule_serial("C201").build().unwrap()).is_ok());
}

#[test]
fn parts_for_specific_capsule_request_builder_fails_with_no_capsule_serial() {
    assert!(space_rx::send(&CapsulePartRequestBuilder::default().build().unwrap()).is_err());
}

#[test]
fn all_core_parts() {
    assert!(space_rx::send(&AllCorePartsRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn all_core_parts_with_params() {
    assert!(space_rx::send(&AllCorePartsRequestBuilder::default().rtls_attempt(true).build().unwrap()).is_ok());
}

#[test]
fn parts_for_specific_core() {
    assert!(space_rx::send(&CorePartRequestBuilder::default().core_serial("B1041").build().unwrap()).is_ok());
}

#[test]
fn parts_for_specific_core_request_builder_fails_with_no_core_serial() {
    assert!(space_rx::send(&CorePartRequestBuilder::default().build().unwrap()).is_err());
}