use ::ApiRequest;
use v2_api::models::capsule::*;

/// Retrieves information about all capsules.  Endpoint is `v2/capsules`.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct AllCapsulesRequest {}

impl ApiRequest for AllCapsulesRequest {
    type Output = Vec<Capsule>;

    fn endpoint(&self) -> String {
        "v2/capsules".to_owned()
    }
}

/// Retrieves information about a specific capsule.  Endpoint is `v2/capsules/{capsule_id}`.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct CapsuleRequest<'a> {
    /// **This parameter is required** - building this request without this parameter will result in an error.
    capsule_id: &'a str
}

impl<'a> ApiRequest for CapsuleRequest<'a> {
    type Output = Capsule;

    fn endpoint(&self) -> String {
        "v2/capsules/".to_owned() + self.capsule_id
    }
}