# space-rx

![Crates.io](https://img.shields.io/crates/v/space_rx.svg)
[![Documentation](https://docs.rs/space-rx/badge.svg?style=flat-square)](https://docs.rs/space-rx/)
![Crates.io](https://img.shields.io/crates/l/space_rx.svg)



Rust wrapper over the unofficial SpaceX API (which can be found [here](https://github.com/r-spacex/SpaceX-API)).

[Documentation](https://docs.rs/space-rx/)

## Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
space_rx = "0.1"
```

## Overview

This crate provides easy to use request builders for all available endpoints in the unofficial SpaceX API.  These request builders return a model after being built and sent, which in turn gives you type-safe, Rustic access to all fields exposed by each of the endpoints.

## Example

Here's an example in which we make a request to the `v2/rockets/{rocket_id}` endpoint - we specify the `rocket_id` as a field in the request builder.

```rust
extern crate space_rx;

use space_rx::v2_api::requests::rocket::*;

fn main() {
    let req = RocketRequestBuilder::default().rocket_id("falcon9").build().unwrap();
    let rocket = space_rx::send(&req).unwrap();

    println!("The Falcon 9 weighs {:?}lbs.  Wow!", rocket.mass.lb);
    println!("The Falcon 9's landing legs are made out of {:?}.", rocket.landing_legs.material.unwrap());
    println!("The Falcon 9's {:?} engine(s) use {:?} and {:?} as propellant.", rocket.engines.number, rocket.engines.propellant_1, rocket.engines.propellant_2);
}
```



