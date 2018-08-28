#[derive(Debug, Deserialize)]
pub struct Length {
    pub feet: Option<f64>,
    pub meters: Option<f64>
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    pub cubic_feet: u16,
    pub cubic_meters: u16
}

#[derive(Debug, Deserialize)]
pub struct Weight {
    pub kg: f64,
    pub lb: f64
}

#[derive(Debug, Deserialize)]
pub struct Force {
    #[serde(rename="kN")]
    pub kn: f64,
    pub lbf: f64
}