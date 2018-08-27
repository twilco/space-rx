extern crate space_rx;

use space_rx::v2_api::requests::launch::*;

#[test]
fn all_launches() {
    assert!(space_rx::send(AllLaunchesRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn all_launches_with_params() {
    assert!(space_rx::send(AllLaunchesRequestBuilder::default().launch_year(2006).build().unwrap()).is_ok());
}

#[test]
fn latest_launch() {
    assert!(space_rx::send(LatestLaunchRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn next_launch() {
    assert!(space_rx::send(NextLaunchRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn past_launches() {
    assert!(space_rx::send(PastLaunchesRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn past_launches_with_params() {
    assert!(space_rx::send(PastLaunchesRequestBuilder::default().launch_year(2006).build().unwrap()).is_ok());
}

#[test]
fn upcoming_launches() {
    assert!(space_rx::send(UpcomingLaunchesRequestBuilder::default().build().unwrap()).is_ok());
}

#[test]
fn upcoming_launches_with_params() {
    assert!(space_rx::send(UpcomingLaunchesRequestBuilder::default().launch_year(2018).build().unwrap()).is_ok());
}