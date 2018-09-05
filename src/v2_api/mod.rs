//! Contains the models (the things returned from requests) and request builders for the V2 API.

/// V2 API models - these are returned after a request builder is built and sent.
pub mod models;
/// V2 API request builders - build requests with all (or none, or some) parameters using the builder pattern.  To see the parameters available for a request, check the associated request builder for parameter methods.  Some requests have no parameters available to use.
pub mod requests;
