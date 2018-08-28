use super::common::*;

#[derive(Debug, Deserialize)]
pub struct Capsule {
    pub active: bool,
    #[serde(rename="type")]
    pub capsule_type: String,
    pub crew_capacity: u16,
    pub diameter: Length,
    pub heat_shield: HeatShield,
    pub height_w_trunk: Length,
    pub id: String,
    pub launch_payload_mass: Weight,
    pub launch_payload_vol: Volume,
    pub name: String,
    pub orbit_duration_yr: u16,
    pub pressurized_capsule: PressurizedCapsule,
    pub return_payload_mass: Weight,
    pub return_payload_vol: Volume,
    pub sidewall_angle_deg: i16,
    pub thrusters: Vec<Thruster>,
    pub trunk: Trunk
}

#[derive(Debug, Deserialize)]
pub struct HeatShield {
    pub dev_partner: String,
    pub material: String,
    pub size_meters: f64,
    pub temp_degrees: i16
}

#[derive(Debug, Deserialize)]
pub struct Thruster {
    pub amount: u16,
    pub fuel_1: String,
    pub fuel_2: String,
    pub pods: u8,
    pub thrust: Force,
    #[serde(rename="type")]
    pub thruster_type: String
}

#[derive(Debug, Deserialize)]
pub struct PressurizedCapsule {
    pub payload_volume: PayloadVolume
}

#[derive(Debug, Deserialize)]
pub struct PayloadVolume {
    pub cubic_feet: u16,
    pub cubic_meters: u16,
}

#[derive(Debug, Deserialize)]
pub struct Trunk {
    pub cargo: Cargo,
    pub trunk_volume: TrunkVolume,
}

#[derive(Debug, Deserialize)]
pub struct Cargo {
    pub solar_array: u8,
    pub unpressurized_cargo: bool
}

#[derive(Debug, Deserialize)]
pub struct TrunkVolume {
    pub cubic_feet: u16,
    pub cubic_meters: u16,
}