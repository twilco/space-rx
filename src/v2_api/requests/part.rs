use std::collections::HashMap;

use ::ApiRequest;
use ::SortDir;

use v2_api::models::part::*;

/// Retrieves info about parts used in all capsules.
///
/// Endpoint is v2/parts/caps.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct AllCapsulePartsRequest<'a> {
    capsule_serial: Option<&'a str>,
    capsule_id: Option<&'a str>,
    status: Option<&'a str>,
    // TODO: The API lists the data type for this (and other dates) as ISO UTC timestamp - can we type check a date valid in this format, vs allowing some String?
    original_launch: Option<&'a str>,
    missions: Option<&'a str>,
    landings: Option<u16>,
    part_type: Option<&'a str>,
    sort: Option<&'a str>,
    order: Option<SortDir>
}

impl<'a> ApiRequest for AllCapsulePartsRequest<'a> {
    type Output = Vec<CapsulePart>;

    fn endpoint(&self) -> String {
        "v2/parts/caps".to_owned()
    }

    fn params(&self) -> Option<HashMap<&str, String>> {
        let mut params = HashMap::new();

        if let Some(capsule_serial) = &self.capsule_serial {
            params.insert("capsule_serial", capsule_serial.to_string());
        }

        if let Some(capsule_id) = self.capsule_id {
            params.insert("capsule_id", capsule_id.to_string());
        }

        if let Some(status) = &self.status {
            params.insert("status", status.to_string());
        }

        if let Some(original_launch) = &self.original_launch {
            params.insert("original_launch", original_launch.to_string());
        }

        if let Some(missions) = &self.missions {
            params.insert("missions", missions.to_string());
        }

        if let Some(landings) = &self.landings {
            params.insert("landings", landings.to_string());
        }

        if let Some(part_type) = &self.part_type {
            params.insert("part_type", part_type.to_string());
        }

        if let Some(sort) = &self.sort {
            params.insert("sort", sort.to_string());
        }

        if let Some(order) = &self.order {
            params.insert("order", order.to_string());
        }

        Some(params)
    }
}

/// Retrieves basic information about parts for a specific capsule.
///
/// Endpoint is v2/parts/caps/{capsule_serial}.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct CapsulePartRequest<'a> {
    capsule_serial: &'a str
}

impl<'a> ApiRequest for CapsulePartRequest<'a> {
    type Output = CapsulePart;

    fn endpoint(&self) -> String {
        "v2/parts/caps/".to_owned() + &self.capsule_serial
    }
}

/// Retrieves info about parts used in all cores.
///
/// Endpoint is v2/parts/cores.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct AllCorePartsRequest<'a> {
    core_serial: Option<&'a str>,
    block: Option<u16>,
    status: Option<&'a str>,
    original_launch: Option<&'a str>,
    missions: Option<&'a str>,
    rtls_attempt: Option<bool>,
    rtls_landings: Option<u16>,
    asds_attempt: Option<bool>,
    asds_landings: Option<u16>,
    water_landing: Option<bool>
}

impl<'a> ApiRequest for AllCorePartsRequest<'a> {
    type Output = Vec<CorePart>;

    fn endpoint(&self) -> String {
        "v2/parts/cores".to_owned()
    }

    fn params(&self) -> Option<HashMap<&str, String>> {
        let mut params = HashMap::new();

        if let Some(core_serial) = &self.core_serial {
            params.insert("core_serial", core_serial.to_string());
        }

        if let Some(block) = self.block {
            params.insert("block", block.to_string());
        }

        if let Some(status) = &self.status {
            params.insert("status", status.to_string());
        }

        if let Some(original_launch) = &self.original_launch {
            params.insert("original_launch", original_launch.to_string());
        }

        if let Some(missions) = &self.missions {
            params.insert("missions", missions.to_string());
        }

        if let Some(rtls_attempt) = &self.rtls_attempt {
            params.insert("rtls_attempt", rtls_attempt.to_string());
        }

        if let Some(rtls_landings) = &self.rtls_landings {
            params.insert("rtls_landings", rtls_landings.to_string());
        }

        if let Some(asds_attempt) = &self.asds_attempt {
            params.insert("asds_attempt", asds_attempt.to_string());
        }

        if let Some(asds_landings) = &self.asds_landings {
            params.insert("asds_landingss", asds_landings.to_string());
        }

        if let Some(water_landing) = &self.water_landing {
            params.insert("water_landing", water_landing.to_string());
        }

        Some(params)
    }
}

/// Retrieves basic information about parts for a specific capsule.
///
/// Endpoint is v2/parts/caps/{core_serial}.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct CorePartRequest<'a> {
    core_serial: &'a str
}

impl<'a> ApiRequest for CorePartRequest<'a> {
    type Output = CorePart;

    fn endpoint(&self) -> String {
        "v2/parts/cores/".to_owned() + &self.core_serial
    }
}

