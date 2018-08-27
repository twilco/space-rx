use std::collections::HashMap;

use ::SortDir;
use ::ApiRequest;
use v2_api::models::launch::*;

// TODO: Make all launch param boilerplate into a macro to avoid repeating the same 200 lines over and over?

/// Retrieves basic information about all past and upcoming launches.
///
/// Endpoint is v2/launches/all.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct AllLaunchesRequest<'a> {
    id: Option<bool>,
    flight_id: Option<&'a str>,
    order: Option<SortDir>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    flight_number: Option<&'a str>,
    launch_year: Option<u16>,
    launch_date_utc: Option<&'a str>,
    launch_date_local: Option<&'a str>,
    launch_id: Option<&'a str>,
    rocket_name: Option<&'a str>,
    rocket_type: Option<&'a str>,
    core_serial: Option<&'a str>,
    cap_serial: Option<&'a str>,
    core_flight: Option<u16>,
    block: Option<u16>,
    core_reuse: Option<bool>,
    side_core1_reuse: Option<bool>,
    side_core2_reuse: Option<bool>,
    fairings_reuse: Option<bool>,
    capsule_reuse: Option<bool>,
    site_id: Option<&'a str>,
    site_name: Option<&'a str>,
    site_name_long: Option<&'a str>,
    payload_id: Option<&'a str>,
    norad_id: Option<&'a str>,
    customer: Option<&'a str>,
    nationality: Option<&'a str>,
    manufacturer: Option<&'a str>,
    payload_type: Option<&'a str>,
    orbit: Option<&'a str>,
    launch_success: Option<&'a str>,
    reused: Option<bool>,
    land_success: Option<bool>,
    landing_type: Option<&'a str>,
    landing_vehicle: Option<&'a str>
}

impl<'a> ApiRequest for AllLaunchesRequest<'a> {
    type Output = Vec<Launch>;

    fn endpoint(&self) -> String {
        "v2/launches/all".to_owned()
    }

    fn params(&self) -> Option<HashMap<&str, String>> {
        let mut params = HashMap::new();

        // TODO: turn all these if let Somes into a macro?  Do they all need to_strings?
        if let Some(return_mongo_id) = &self.id {
            params.insert("id", return_mongo_id.to_string());
        }

        if let Some(flight_id) = &self.flight_id {
            params.insert("flight_id", flight_id.to_string());
        }

        if let Some(order) = &self.order {
            params.insert("order", order.to_string());
        }

        if let Some(start) = &self.start {
            params.insert("start", start.to_string());
        }

        if let Some(end) = &self.end {
            params.insert("end", end.to_string());
        }

        if let Some(flight_number) = &self.flight_number {
            params.insert("flight_number", flight_number.to_string());
        }

        if let Some(launch_year) = &self.launch_year {
            params.insert("launch_year", launch_year.to_string());
        }

        if let Some(launch_date_utc) = &self.launch_date_utc {
            params.insert("launch_date_utc", launch_date_utc.to_string());
        }

        if let Some(launch_date_local) = &self.launch_date_local {
            params.insert("launch_date_local", launch_date_local.to_string());
        }

        if let Some(launch_id) = &self.launch_id {
            params.insert("launch_id", launch_id.to_string());
        }

        if let Some(rocket_name) = &self.rocket_name {
            params.insert("rocket_name", rocket_name.to_string());
        }

        if let Some(rocket_type) = &self.rocket_type {
            params.insert("rocket_type", rocket_type.to_string());
        }

        if let Some(core_serial) = &self.core_serial {
            params.insert("core_serial", core_serial.to_string());
        }

        if let Some(cap_serial) = &self.cap_serial {
            params.insert("cap_serial", cap_serial.to_string());
        }

        if let Some(core_flight) = &self.core_flight {
            params.insert("core_flight", core_flight.to_string());
        }

        if let Some(block) = &self.block {
            params.insert("block", block.to_string());
        }

        if let Some(core_reuse) = &self.core_reuse {
            params.insert("core_reuse", core_reuse.to_string());
        }

        if let Some(side_core1_reuse) = &self.side_core1_reuse {
            params.insert("side_core1_reuse", side_core1_reuse.to_string());
        }

        if let Some(side_core2_reuse) = &self.side_core2_reuse {
            params.insert("side_core2_reuse", side_core2_reuse.to_string());
        }

        if let Some(fairings_reuse) = &self.fairings_reuse {
            params.insert("fairings_reuse", fairings_reuse.to_string());
        }

        if let Some(capsule_reuse) = &self.capsule_reuse {
            params.insert("capsule_reuse", capsule_reuse.to_string());
        }

        if let Some(site_id) = &self.site_id {
            params.insert("site_id", site_id.to_string());
        }

        if let Some(site_name) = &self.site_name {
            params.insert("site_name", site_name.to_string());
        }

        if let Some(site_name_long) = &self.site_name_long {
            params.insert("site_name_long", site_name_long.to_string());
        }

        if let Some(payload_id) = &self.payload_id {
            params.insert("payload_id", payload_id.to_string());
        }

        if let Some(norad_id) = &self.norad_id {
            params.insert("norad_id", norad_id.to_string());
        }

        if let Some(customer) = &self.customer {
            params.insert("customer", customer.to_string());
        }

        if let Some(nationality) = &self.nationality {
            params.insert("nationality", nationality.to_string());
        }

        if let Some(manufacturer) = &self.manufacturer {
            params.insert("manufacturer", manufacturer.to_string());
        }

        if let Some(payload_type) = &self.payload_type {
            params.insert("payload_type", payload_type.to_string());
        }

        if let Some(orbit) = &self.orbit {
            params.insert("orbit", orbit.to_string());
        }

        if let Some(launch_success) = &self.launch_success {
            params.insert("launch_success", launch_success.to_string());
        }

        if let Some(reused) = &self.reused {
            params.insert("reused", reused.to_string());
        }

        if let Some(land_success) = &self.land_success {
            params.insert("land_success", land_success.to_string());
        }

        if let Some(landing_type) = &self.landing_type {
            params.insert("landing_type", landing_type.to_string());
        }

        if let Some(landing_vehicle) = &self.landing_vehicle {
            params.insert("landing_vehicle", landing_vehicle.to_string());
        }

        Some(params)
    }
}

/// Retrieves basic information about all past launches.
///
/// Endpoint is v2/launches.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct PastLaunchesRequest<'a> {
    id: Option<bool>,
    flight_id: Option<&'a str>,
    order: Option<SortDir>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    flight_number: Option<&'a str>,
    launch_year: Option<u16>,
    launch_date_utc: Option<&'a str>,
    launch_date_local: Option<&'a str>,
    launch_id: Option<&'a str>,
    rocket_name: Option<&'a str>,
    rocket_type: Option<&'a str>,
    core_serial: Option<&'a str>,
    cap_serial: Option<&'a str>,
    core_flight: Option<u16>,
    block: Option<u16>,
    core_reuse: Option<bool>,
    side_core1_reuse: Option<bool>,
    side_core2_reuse: Option<bool>,
    fairings_reuse: Option<bool>,
    capsule_reuse: Option<bool>,
    site_id: Option<&'a str>,
    site_name: Option<&'a str>,
    site_name_long: Option<&'a str>,
    payload_id: Option<&'a str>,
    norad_id: Option<&'a str>,
    customer: Option<&'a str>,
    nationality: Option<&'a str>,
    manufacturer: Option<&'a str>,
    payload_type: Option<&'a str>,
    orbit: Option<&'a str>,
    launch_success: Option<&'a str>,
    reused: Option<bool>,
    land_success: Option<bool>,
    landing_type: Option<&'a str>,
    landing_vehicle: Option<&'a str>
}

impl<'a> ApiRequest for PastLaunchesRequest<'a> {
    type Output = Vec<Launch>;

    fn endpoint(&self) -> String {
        "v2/launches".to_owned()
    }

    fn params(&self) -> Option<HashMap<&str, String>> {
        let mut params = HashMap::new();

        // TODO: turn all these if let Somes into a macro?  Do they all need to_strings?
        if let Some(return_mongo_id) = &self.id {
            params.insert("id", return_mongo_id.to_string());
        }

        if let Some(flight_id) = &self.flight_id {
            params.insert("flight_id", flight_id.to_string());
        }

        if let Some(order) = &self.order {
            params.insert("order", order.to_string());
        }

        if let Some(start) = &self.start {
            params.insert("start", start.to_string());
        }

        if let Some(end) = &self.end {
            params.insert("end", end.to_string());
        }

        if let Some(flight_number) = &self.flight_number {
            params.insert("flight_number", flight_number.to_string());
        }

        if let Some(launch_year) = &self.launch_year {
            params.insert("launch_year", launch_year.to_string());
        }

        if let Some(launch_date_utc) = &self.launch_date_utc {
            params.insert("launch_date_utc", launch_date_utc.to_string());
        }

        if let Some(launch_date_local) = &self.launch_date_local {
            params.insert("launch_date_local", launch_date_local.to_string());
        }

        if let Some(launch_id) = &self.launch_id {
            params.insert("launch_id", launch_id.to_string());
        }

        if let Some(rocket_name) = &self.rocket_name {
            params.insert("rocket_name", rocket_name.to_string());
        }

        if let Some(rocket_type) = &self.rocket_type {
            params.insert("rocket_type", rocket_type.to_string());
        }

        if let Some(core_serial) = &self.core_serial {
            params.insert("core_serial", core_serial.to_string());
        }

        if let Some(cap_serial) = &self.cap_serial {
            params.insert("cap_serial", cap_serial.to_string());
        }

        if let Some(core_flight) = &self.core_flight {
            params.insert("core_flight", core_flight.to_string());
        }

        if let Some(block) = &self.block {
            params.insert("block", block.to_string());
        }

        if let Some(core_reuse) = &self.core_reuse {
            params.insert("core_reuse", core_reuse.to_string());
        }

        if let Some(side_core1_reuse) = &self.side_core1_reuse {
            params.insert("side_core1_reuse", side_core1_reuse.to_string());
        }

        if let Some(side_core2_reuse) = &self.side_core2_reuse {
            params.insert("side_core2_reuse", side_core2_reuse.to_string());
        }

        if let Some(fairings_reuse) = &self.fairings_reuse {
            params.insert("fairings_reuse", fairings_reuse.to_string());
        }

        if let Some(capsule_reuse) = &self.capsule_reuse {
            params.insert("capsule_reuse", capsule_reuse.to_string());
        }

        if let Some(site_id) = &self.site_id {
            params.insert("site_id", site_id.to_string());
        }

        if let Some(site_name) = &self.site_name {
            params.insert("site_name", site_name.to_string());
        }

        if let Some(site_name_long) = &self.site_name_long {
            params.insert("site_name_long", site_name_long.to_string());
        }

        if let Some(payload_id) = &self.payload_id {
            params.insert("payload_id", payload_id.to_string());
        }

        if let Some(norad_id) = &self.norad_id {
            params.insert("norad_id", norad_id.to_string());
        }

        if let Some(customer) = &self.customer {
            params.insert("customer", customer.to_string());
        }

        if let Some(nationality) = &self.nationality {
            params.insert("nationality", nationality.to_string());
        }

        if let Some(manufacturer) = &self.manufacturer {
            params.insert("manufacturer", manufacturer.to_string());
        }

        if let Some(payload_type) = &self.payload_type {
            params.insert("payload_type", payload_type.to_string());
        }

        if let Some(orbit) = &self.orbit {
            params.insert("orbit", orbit.to_string());
        }

        if let Some(launch_success) = &self.launch_success {
            params.insert("launch_success", launch_success.to_string());
        }

        if let Some(reused) = &self.reused {
            params.insert("reused", reused.to_string());
        }

        if let Some(land_success) = &self.land_success {
            params.insert("land_success", land_success.to_string());
        }

        if let Some(landing_type) = &self.landing_type {
            params.insert("landing_type", landing_type.to_string());
        }

        if let Some(landing_vehicle) = &self.landing_vehicle {
            params.insert("landing_vehicle", landing_vehicle.to_string());
        }

        Some(params)
    }
}

/// Retrieves basic information about the latest launch.
///
/// Endpoint is v2/launches/latest.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct LatestLaunchRequest {}

impl ApiRequest for LatestLaunchRequest {
    type Output = Launch;

    fn endpoint(&self) -> String {
        "v2/launches/latest".to_owned()
    }
}

/// Retrieves basic information about the next launch.
///
/// Endpoint is v2/launches/next.
#[derive(Builder, Debug, Default)]
#[builder(default)]
pub struct NextLaunchRequest {}

impl ApiRequest for NextLaunchRequest {
    type Output = Launch;

    fn endpoint(&self) -> String {
        "v2/launches/next".to_owned()
    }
}

/// Retrieves basic information about all upcoming launches.
///
/// Endpoint is v2/launches/upcoming.
#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
#[builder(default)]
pub struct UpcomingLaunchesRequest<'a> {
    id: Option<bool>,
    flight_id: Option<&'a str>,
    order: Option<SortDir>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    flight_number: Option<&'a str>,
    launch_year: Option<u16>,
    launch_date_utc: Option<&'a str>,
    launch_date_local: Option<&'a str>,
    launch_id: Option<&'a str>,
    rocket_name: Option<&'a str>,
    rocket_type: Option<&'a str>,
    core_serial: Option<&'a str>,
    cap_serial: Option<&'a str>,
    core_flight: Option<u16>,
    block: Option<u16>,
    core_reuse: Option<bool>,
    side_core1_reuse: Option<bool>,
    side_core2_reuse: Option<bool>,
    fairings_reuse: Option<bool>,
    capsule_reuse: Option<bool>,
    site_id: Option<&'a str>,
    site_name: Option<&'a str>,
    site_name_long: Option<&'a str>,
    payload_id: Option<&'a str>,
    norad_id: Option<&'a str>,
    customer: Option<&'a str>,
    nationality: Option<&'a str>,
    manufacturer: Option<&'a str>,
    payload_type: Option<&'a str>,
    orbit: Option<&'a str>,
    launch_success: Option<&'a str>,
    reused: Option<bool>,
    land_success: Option<bool>,
    landing_type: Option<&'a str>,
    landing_vehicle: Option<&'a str>
}

impl<'a> ApiRequest for UpcomingLaunchesRequest<'a> {
    type Output = Vec<Launch>;

    fn endpoint(&self) -> String {
        "v2/launches/upcoming".to_owned()
    }

    fn params(&self) -> Option<HashMap<&str, String>> {
        let mut params = HashMap::new();

        // TODO: turn all these if let Somes into a macro?  Do they all need to_strings?
        if let Some(return_mongo_id) = &self.id {
            params.insert("id", return_mongo_id.to_string());
        }

        if let Some(flight_id) = &self.flight_id {
            params.insert("flight_id", flight_id.to_string());
        }

        if let Some(order) = &self.order {
            params.insert("order", order.to_string());
        }

        if let Some(start) = &self.start {
            params.insert("start", start.to_string());
        }

        if let Some(end) = &self.end {
            params.insert("end", end.to_string());
        }

        if let Some(flight_number) = &self.flight_number {
            params.insert("flight_number", flight_number.to_string());
        }

        if let Some(launch_year) = &self.launch_year {
            params.insert("launch_year", launch_year.to_string());
        }

        if let Some(launch_date_utc) = &self.launch_date_utc {
            params.insert("launch_date_utc", launch_date_utc.to_string());
        }

        if let Some(launch_date_local) = &self.launch_date_local {
            params.insert("launch_date_local", launch_date_local.to_string());
        }

        if let Some(launch_id) = &self.launch_id {
            params.insert("launch_id", launch_id.to_string());
        }

        if let Some(rocket_name) = &self.rocket_name {
            params.insert("rocket_name", rocket_name.to_string());
        }

        if let Some(rocket_type) = &self.rocket_type {
            params.insert("rocket_type", rocket_type.to_string());
        }

        if let Some(core_serial) = &self.core_serial {
            params.insert("core_serial", core_serial.to_string());
        }

        if let Some(cap_serial) = &self.cap_serial {
            params.insert("cap_serial", cap_serial.to_string());
        }

        if let Some(core_flight) = &self.core_flight {
            params.insert("core_flight", core_flight.to_string());
        }

        if let Some(block) = &self.block {
            params.insert("block", block.to_string());
        }

        if let Some(core_reuse) = &self.core_reuse {
            params.insert("core_reuse", core_reuse.to_string());
        }

        if let Some(side_core1_reuse) = &self.side_core1_reuse {
            params.insert("side_core1_reuse", side_core1_reuse.to_string());
        }

        if let Some(side_core2_reuse) = &self.side_core2_reuse {
            params.insert("side_core2_reuse", side_core2_reuse.to_string());
        }

        if let Some(fairings_reuse) = &self.fairings_reuse {
            params.insert("fairings_reuse", fairings_reuse.to_string());
        }

        if let Some(capsule_reuse) = &self.capsule_reuse {
            params.insert("capsule_reuse", capsule_reuse.to_string());
        }

        if let Some(site_id) = &self.site_id {
            params.insert("site_id", site_id.to_string());
        }

        if let Some(site_name) = &self.site_name {
            params.insert("site_name", site_name.to_string());
        }

        if let Some(site_name_long) = &self.site_name_long {
            params.insert("site_name_long", site_name_long.to_string());
        }

        if let Some(payload_id) = &self.payload_id {
            params.insert("payload_id", payload_id.to_string());
        }

        if let Some(norad_id) = &self.norad_id {
            params.insert("norad_id", norad_id.to_string());
        }

        if let Some(customer) = &self.customer {
            params.insert("customer", customer.to_string());
        }

        if let Some(nationality) = &self.nationality {
            params.insert("nationality", nationality.to_string());
        }

        if let Some(manufacturer) = &self.manufacturer {
            params.insert("manufacturer", manufacturer.to_string());
        }

        if let Some(payload_type) = &self.payload_type {
            params.insert("payload_type", payload_type.to_string());
        }

        if let Some(orbit) = &self.orbit {
            params.insert("orbit", orbit.to_string());
        }

        if let Some(launch_success) = &self.launch_success {
            params.insert("launch_success", launch_success.to_string());
        }

        if let Some(reused) = &self.reused {
            params.insert("reused", reused.to_string());
        }

        if let Some(land_success) = &self.land_success {
            params.insert("land_success", land_success.to_string());
        }

        if let Some(landing_type) = &self.landing_type {
            params.insert("landing_type", landing_type.to_string());
        }

        if let Some(landing_vehicle) = &self.landing_vehicle {
            params.insert("landing_vehicle", landing_vehicle.to_string());
        }

        Some(params)
    }
}