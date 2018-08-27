#[derive(Debug, Deserialize)]
pub struct Launchpad {
    details: String,
    full_name: String,
    id: String,
    location: LaunchpadLocation,
    status: String,
    vehicles_launched: Vec<String>
}

#[derive(Debug, Deserialize)]
struct LaunchpadLocation {
    latitude: f64,
    longitude: f64,
    name: String,
    region: String
}