#[derive(Debug, Deserialize)]
pub struct Length {
    feet: Option<f64>,
    meters: Option<f64>
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    cubic_feet: u16,
    cubic_meters: u16
}

#[derive(Debug, Deserialize)]
pub struct Weight {
    kg: f64,
    lb: f64
}

#[derive(Debug, Deserialize)]
pub struct Force {
    #[serde(rename="kN")]
    kn: f64,
    lbf: f64
}