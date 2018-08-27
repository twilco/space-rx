use super::common::*;

#[derive(Debug, Deserialize)]
pub struct Capsule {
    active: bool,
    #[serde(rename="type")]
    capsule_type: String,
    crew_capacity: u16,
    diameter: Length,
    heat_shield: HeatShield,
    height_w_trunk: Length,
    id: String,
    launch_payload_mass: Weight,
    launch_payload_vol: Volume,
    name: String,
    orbit_duration_yr: u16,
    pressurized_capsule: PressurizedCapsule,
    return_payload_mass: Weight,
    return_payload_vol: Volume,
    sidewall_angle_deg: i16,
    thrusters: Vec<Thruster>,
    trunk: Trunk
}

#[derive(Debug, Deserialize)]
struct HeatShield {
    dev_partner: String,
    material: String,
    size_meters: f64,
    temp_degrees: i16
}

#[derive(Debug, Deserialize)]
struct Thruster {
    amount: u16,
    fuel_1: String,
    fuel_2: String,
    pods: u8,
    thrust: Force,
    #[serde(rename="type")]
    thruster_type: String
}

#[derive(Debug, Deserialize)]
struct PressurizedCapsule {
    payload_volume: PayloadVolume
}

#[derive(Debug, Deserialize)]
struct PayloadVolume {
    cubic_feet: u16,
    cubic_meters: u16,
}

#[derive(Debug, Deserialize)]
struct Trunk {
    cargo: Cargo,
    trunk_volume: TrunkVolume,
}

#[derive(Debug, Deserialize)]
struct Cargo {
    solar_array: u8,
    unpressurized_cargo: bool
}

#[derive(Debug, Deserialize)]
struct TrunkVolume {
    cubic_feet: u16,
    cubic_meters: u16,
}