//! # space_rx
//!
//! `space_rx` is a crate that provides easy to use request builders for all available endpoints in
//! [the unofficial SpaceX API](https://github.com/r-spacex/SpaceX-API). These request builders return
//! a model after being built and sent, which in turn gives you type-safe, Rustacious access to all
//! fields exposed by each of the endpoints.
//!
//! To get right to the available request builders, scroll to the bottom of this page, select your API version of choice, and then head into the requests module.  For example, [v2_api/requests](v2_api/requests/index.html).
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! space_rx = "0.2"
//! ```
//!
//! ## Examples
//!
//! Here's an example in which we make a request to the `v2/rockets/{rocket_id}` - we specify the `rocket_id` as a field in the request builder.
//!
//! ```rust
//! extern crate space_rx;
//!
//! use space_rx::v2_api::requests::rocket::*;
//!
//! fn main() {
//!     let req = RocketRequestBuilder::default().rocket_id("falcon9").build().unwrap();
//!     let rocket = space_rx::send(&req).unwrap();
//!
//!     println!("The Falcon 9 weighs {:?}lbs.  Wow!", rocket.mass.lb);
//!     println!("The Falcon 9's landing legs are made out of {:?}.", rocket.landing_legs.material.unwrap());
//!     println!("The Falcon 9's {:?} engines used {:?} and {:?} as propellant.", rocket.engines.number, rocket.engines.propellant_1, rocket.engines.propellant_2);
//! }
//! ```
//!
//! How's [Starman](https://en.wikipedia.org/wiki/Elon_Musk%27s_Tesla_Roadster) doing?  Let's find out!
//!
//! ```rust
//! extern crate space_rx;
//!
//! use space_rx::v2_api::requests::info::*;
//!
//! fn main() {
//!     let req = RoadsterInfoRequestBuilder::default().build().unwrap();
//!     let roadster_info = space_rx::send(&req).unwrap();
//!
//!     println!("Here's how Starman is doing: {:?}", roadster_info);
//! }
//! ```
//!
//! The `launch` style endpoints have lots of potential parameters to filter requests.  Let's try some of those.
//! ```rust
//! extern crate space_rx;
//!
//! use space_rx::SortDir;
//! use space_rx::v2_api::requests::launch::*;
//!
//! fn main() {
//!     let req = AllLaunchesRequestBuilder::default().reused(true)
//!                                                   .start("2010-06-22")
//!                                                   .customer("Telkom")
//!                                                   .site_name("CCAFS SLC 40")
//!                                                   .order(SortDir::DESC)
//!                                                   .build()
//!                                                   .unwrap();
//!
//!     let launches = space_rx::send(&req).unwrap();
//!
//!     println!("{:?}", launches);
//! }
//! ```
//!
//! And here we get some information about the Dragon 1 capsule.
//!
//! ```rust
//! extern crate space_rx;
//!
//! use space_rx::v2_api::requests::capsule::*;
//!
//! fn main() {
//!     let req = CapsuleRequestBuilder::default().capsule_id("dragon1").build().unwrap();
//!     let capsule = space_rx::send(&req).unwrap();
//!
//!     println!("The Dragon 1 capsule can hold {:?} people.", capsule.crew_capacity);
//!     println!("How much junk does the Dragon 1 capsule have in the trunk?  Well, this much: {:?}.", capsule.trunk);
//! }
//! ```
//!
//! Curious to know what launchpads SpaceX uses?  Let's try that.
//!
//! ```rust
//! extern crate space_rx;
//!
//! use space_rx::v2_api::requests::launchpad::*;
//!
//! fn main() {
//!     let req = LaunchpadRequestBuilder::default().launchpad_id("ksc_lc_39a").build().unwrap();
//!     let kennedy_space_center_39a_launchpad = space_rx::send(&req).unwrap();
//!
//!     println!("{:?} has launched these rockets: {:?}.", kennedy_space_center_39a_launchpad.full_name, kennedy_space_center_39a_launchpad.vehicles_launched);
//!     println!("{:?} is located at {:?}.", kennedy_space_center_39a_launchpad.full_name, kennedy_space_center_39a_launchpad.location);
//! }
//! ```
//!
//! How about the parts used in all capsules?  We can find that, too.
//!
//! ```rust
//! extern crate space_rx;
//!
//! use space_rx::v2_api::requests::part::*;
//!
//! fn main() {
//!     let req = AllCapsulePartsRequestBuilder::default().build().unwrap();
//!     let capsule_parts = space_rx::send(&req).unwrap();
//!
//!     println!("Here are the parts used in SpaceX capsules: {:?}", capsule_parts);
//! }
//! ```

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
#[macro_use]
extern crate derive_builder;
extern crate failure;
extern crate url;

use std::fmt;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use reqwest::Client;
use url::Url;

pub mod v2_api;

/// Behavior for [`send`]able SpaceX API requests.
pub trait ApiRequest {
    type Output: DeserializeOwned;

    fn endpoint(&self) -> String;

    fn params(&self) -> Option<HashMap<&str, String>> {
        None
    }
}

/// Sends off a built SpaceX API request, and returns the model associated with the request type.
pub fn send<T: ApiRequest>(request: &T) -> Result<T::Output, failure::Error> {
    let base = &("https://api.spacexdata.com/".to_owned() + &request.endpoint());
    let url = match request.params() {
        Some(map) => Url::parse_with_params(base, map)?,
        None => Url::parse(base)?
    };

    let client = Client::new();
    let mut request = client.get(url.as_str());
    let mut response = request.send()?;
    Ok(response.json()?)
}

/// Direction in which to sort returned results - can be used as a parameter in some request types.
#[derive(Clone, Debug)]
pub enum SortDir {
    /// Ascending sort
    ASC,
    /// Descending sort
    DESC
}

impl fmt::Display for SortDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

