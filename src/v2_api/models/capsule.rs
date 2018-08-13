use super::common::*;

#[derive(Debug, Deserialize)]
pub struct Capsule {
    id: String,
    name: String,
    #[serde(rename="type")]
    capsule_type: String,
    active: bool,
    crew_capacity: u16,
    sidewall_angle_deg: i16,
    orbit_duration_yr: u16,
    heat_shield: HeatShield,
    thrusters: Vec<Thruster>,
    launch_payload_mass: Weight,
    launch_payload_vol: Volume,
    return_payload_mass: Weight,
    return_payload_vol: Volume,
    pressurized_capsule: PressurizedCapsule,
    trunk: Trunk,
    height_w_trunk: Length,
    diameter: Length
}

#[derive(Debug, Deserialize)]
struct HeatShield {
    material: String,
    size_meters: f64,
    temp_degrees: i16,
    dev_partner: String
}

#[derive(Debug, Deserialize)]
struct Thruster {
    #[serde(rename="type")]
    thruster_type: String,
    amount: u16,
    pods: u8,
    fuel_1: String,
    fuel_2: String,
    thrust: Force
}

#[derive(Debug, Deserialize)]
struct PressurizedCapsule {
    payload_volume: PayloadVolume
}

#[derive(Debug, Deserialize)]
struct PayloadVolume {
    cubic_meters: u16,
    cubic_feet: u16
}

#[derive(Debug, Deserialize)]
struct Trunk {
    trunk_volume: TrunkVolume,
    cargo: Cargo
}

#[derive(Debug, Deserialize)]
struct Cargo {
    solar_array: u8,
    unpressurized_cargo: bool
}

#[derive(Debug, Deserialize)]
struct TrunkVolume {
    cubic_meters: u16,
    cubic_feet: u16
}