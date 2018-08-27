extern crate space_rx;

use space_rx::v2_api::requests::launchpad::*;

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
