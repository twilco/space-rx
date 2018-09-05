extern crate space_rx;

use space_rx::v2_api::requests::part::*;

fn main() {
    let req = AllCapsulePartsRequestBuilder::default().build().unwrap();
    let capsule_parts = space_rx::send(&req).unwrap();

    println!("Here are the parts used in SpaceX capsules: {:?}", capsule_parts);
}