use super::common::*;

#[derive(Debug, Deserialize)]
pub struct Rocket {
    active: bool,
    boosters: u16,
    company: String,
    cost_per_launch: u64,
    country: String,
    description: String,
    diameter: Length,
    engines: Engines,
    first_flight: String,
    first_stage: FirstStage,
    height: Length,
    id: String,
    landing_legs: LandingLegs,
    mass: Weight,
    name: String,
    payload_weights: Vec<PayloadWeight>,
    #[serde(rename="type")]
    rocket_type: String,
    stages: u16,
    success_rate_pct: u8,
    second_stage: SecondStage
}

#[derive(Debug, Deserialize)]
struct PayloadWeight {
    id: String,
    kg: f64,
    lb: f64,
    name: String
}


#[derive(Debug, Deserialize)]
struct FirstStage {
    burn_time_sec: u32,
    engines: u16,
    fuel_amount_tons: f64,
    reusable: bool,
    thrust_sea_level: Force,
    thrust_vacuum: Force
}

#[derive(Debug, Deserialize)]
struct SecondStage {
    burn_time_sec: u32,
    engines: u16,
    fuel_amount_tons: Option<f64>,
    payloads: SecondStagePayloads,
    thrust: Force
}

#[derive(Debug, Deserialize)]
struct SecondStagePayloads {
    composite_fairing: CompositeFairing,
    option_1: Option<String>,
    option_2: Option<String>
}

#[derive(Debug, Deserialize)]
struct CompositeFairing {
    diameter: Length,
    height: Length
}

#[derive(Debug, Deserialize)]
struct Engines {
    engine_loss_max: Option<u16>,
    #[serde(rename="type")]
    engine_type: String,
    layout: Option<String>,
    propellant_1: String,
    propellant_2: String,
    number: u16,
    thrust_sea_level: Force,
    thrust_to_weight: Option<f64>,
    thrust_vacuum: Force,
    version: String
}

#[derive(Debug, Deserialize)]
struct LandingLegs {
    material: Option<String>,
    number: u8
}