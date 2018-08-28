#[derive(Debug, Deserialize)]
pub struct Launchpad {
    pub details: String,
    pub full_name: String,
    pub id: String,
    pub location: LaunchpadLocation,
    pub status: String,
    pub vehicles_launched: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct LaunchpadLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub region: String
}