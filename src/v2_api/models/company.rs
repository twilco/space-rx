use ::ApiRequest;

#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct CompanyRequest {}

impl ApiRequest for CompanyRequest {
    type Output = CompanyInfo;

    fn endpoint(&self) -> String {
        "v2/info".to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct CompanyInfo {
    name: String,
    founder: String,
    founded: u16,
    employees: u32,
    vehicles: u32,
    launch_sites: u16,
    test_sites: u16,
    ceo: String,
    cto: String,
    coo: String,
    cto_propulsion: String,
    valuation: u64,
    headquarters: Headquarters,
    summary: String
}

#[derive(Debug, Deserialize)]
struct Headquarters {
    address: String,
    city: String,
    state: String
}