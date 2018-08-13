#[derive(Debug, Deserialize)]
pub struct Launchpad {
    id: String,
    full_name: String,
    status: String,
    location: LaunchpadLocation,
    vehicles_launched: Vec<String>,
    details: String
}

#[derive(Debug, Deserialize)]
struct LaunchpadLocation {
    name: String,
    region: String,
    latitude: f64,
    longitude: f64
}