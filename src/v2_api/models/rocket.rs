use super::common::*;

#[derive(Debug, Deserialize)]
pub struct Rocket {
    pub active: bool,
    pub boosters: u16,
    pub company: String,
    pub cost_per_launch: u64,
    pub country: String,
    pub description: String,
    pub diameter: Length,
    pub engines: Engines,
    pub first_flight: String,
    pub first_stage: FirstStage,
    pub height: Length,
    pub id: String,
    pub landing_legs: LandingLegs,
    pub mass: Weight,
    pub name: String,
    pub payload_weights: Vec<PayloadWeight>,
    #[serde(rename="type")]
    pub rocket_type: String,
    pub stages: u16,
    pub success_rate_pct: u8,
    pub second_stage: SecondStage
}

#[derive(Debug, Deserialize)]
pub struct PayloadWeight {
    pub id: String,
    pub kg: f64,
    pub lb: f64,
    pub name: String
}


#[derive(Debug, Deserialize)]
pub struct FirstStage {
    pub burn_time_sec: u32,
    pub engines: u16,
    pub fuel_amount_tons: f64,
    pub reusable: bool,
    pub thrust_sea_level: Force,
    pub thrust_vacuum: Force
}

#[derive(Debug, Deserialize)]
pub struct SecondStage {
    pub burn_time_sec: u32,
    pub engines: u16,
    pub fuel_amount_tons: Option<f64>,
    pub payloads: SecondStagePayloads,
    pub thrust: Force
}

#[derive(Debug, Deserialize)]
pub struct SecondStagePayloads {
    pub composite_fairing: CompositeFairing,
    pub option_1: Option<String>,
    pub option_2: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CompositeFairing {
    pub diameter: Length,
    pub height: Length
}

#[derive(Debug, Deserialize)]
pub struct Engines {
    pub engine_loss_max: Option<u16>,
    #[serde(rename="type")]
    pub engine_type: String,
    pub layout: Option<String>,
    pub propellant_1: String,
    pub propellant_2: String,
    pub number: u16,
    pub thrust_sea_level: Force,
    pub thrust_to_weight: Option<f64>,
    pub thrust_vacuum: Force,
    pub version: String
}

#[derive(Debug, Deserialize)]
pub struct LandingLegs {
    pub material: Option<String>,
    pub number: u8
}