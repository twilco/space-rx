use ::ApiRequest;
use v2_api::models::launchpad::*;

/// Retrieves information about all launchpads.  Endpoint is `v2/launchpads`.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct AllLaunchpadsRequest {}

impl ApiRequest for AllLaunchpadsRequest {
    type Output = Vec<Launchpad>;

    fn endpoint(&self) -> String {
        "v2/launchpads".to_owned()
    }
}

/// Retrieves information about a specific launchpad.  Endpoint is `v2/launchpads/{launchpad_id}`.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct LaunchpadRequest<'a> {
    /// **This parameter is required** - building this request without this parameter will result in an error.
    launchpad_id: &'a str
}

impl<'a> ApiRequest for LaunchpadRequest<'a> {
    type Output = Launchpad;

    fn endpoint(&self) -> String {
        "v2/launchpads/".to_owned() + self.launchpad_id
    }
}