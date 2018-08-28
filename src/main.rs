extern crate space_rx;

use space_rx::v2_api::requests::rocket::*;

fn main() {
    let req = RocketRequestBuilder::default().rocket_id("falcon9").build().unwrap();
    let rocket = space_rx::send(&req).unwrap();

    println!("The Falcon 9 weighs {:?}lbs.  Wow!", rocket.mass.lb);
    println!("The Falcon 9's landing legs are made out of {:?}.", rocket.landing_legs.material.unwrap());
    println!("The Falcon 9's {:?} engines used {:?} and {:?} as propellant.", rocket.engines.number, rocket.engines.propellant_1, rocket.engines.propellant_2);
}