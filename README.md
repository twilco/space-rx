# space-rx

Rust wrapper over the unofficial SpaceX API (which can be found [here](https://github.com/r-spacex/SpaceX-API)).

## Installation

Add this to your `Cargo.toml` file:

[dependencies]
space_rx = "0.1"

## Overview

This crate provides easy to use request builders for all available endpoints in the unofficial SpaceX API.  These request builders return a model, which in turn gives you type-safe, Rustic access to all fields exposed by each of the endpoints.



