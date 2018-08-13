use super::common::*;

#[derive(Debug, Deserialize)]
pub struct Rocket {
    id: String,
    name: String,
    #[serde(rename="type")]
    rocket_type: String,
    active: bool,
    stages: u16,
    boosters: u16,
    cost_per_launch: u64,
    success_rate_pct: u8,
    first_flight: String,
    country: String,
    company: String,
    height: Length,
    diameter: Length,
    mass: Weight,
    payload_weights: Vec<PayloadWeight>,
    first_stage: FirstStage,
    second_stage: SecondStage,
    engines: Engines,
    landing_legs: LandingLegs,
    description: String
}

#[derive(Debug, Deserialize)]
struct PayloadWeight {
    id: String,
    name: String,
    kg: f64,
    lb: f64
}


#[derive(Debug, Deserialize)]
struct FirstStage {
    reusable: bool,
    engines: u16,
    fuel_amount_tons: f64,
    burn_time_sec: u32,
    thrust_sea_level: Force,
    thrust_vacuum: Force
}

#[derive(Debug, Deserialize)]
struct SecondStage {
    engines: u16,
    fuel_amount_tons: Option<f64>,
    burn_time_sec: u32,
    thrust: Force,
    payloads: SecondStagePayloads
}

#[derive(Debug, Deserialize)]
struct SecondStagePayloads {
    option_1: Option<String>,
    option_2: Option<String>,
    composite_fairing: CompositeFairing
}

#[derive(Debug, Deserialize)]
struct CompositeFairing {
    height: Length,
    diameter: Length
}

#[derive(Debug, Deserialize)]
struct Engines {
    number: u16,
    #[serde(rename="type")]
    engine_type: String,
    version: String,
    layout: Option<String>,
    engine_loss_max: Option<u16>,
    propellant_1: String,
    propellant_2: String,
    thrust_sea_level: Force,
    thrust_vacuum: Force,
    thrust_to_weight: Option<f64>
}

#[derive(Debug, Deserialize)]
struct LandingLegs {
    number: u8,
    material: Option<String>
}