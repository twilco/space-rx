use ::ApiRequest;
use v2_api::models::rocket::*;

/// Retrieves information about all SpaceX rockets.  Endpoint is `v2/rockets`.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct AllRocketsRequest {}

impl ApiRequest for AllRocketsRequest {
    type Output = Vec<Rocket>;

    fn endpoint(&self) -> String {
        "v2/rockets".to_owned()
    }
}

/// Retrieves information about a specific SpaceX rocket.  Endpoint is `v2/rockets/{rocket_id}`.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct RocketRequest<'a> {
    /// **This parameter is required** - building this request without this parameter will result in an error.
    rocket_id: &'a str
}

impl<'a> ApiRequest for RocketRequest<'a> {
    type Output = Rocket;

    fn endpoint(&self) -> String {
        "v2/rockets/".to_owned() + self.rocket_id
    }
}