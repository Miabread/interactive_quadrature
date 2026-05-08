mod algorithm;
mod app;
pub use app::App;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Endpoints {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InterpolationPoint {
    pub x: f64,
    pub y: f64,
    pub area: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Algorithm {
    NewtonCotes,
}

impl Algorithm {
    pub const VARIANTS: &[Algorithm] = &[Algorithm::NewtonCotes];

    pub fn text(&self) -> &'static str {
        match self {
            Algorithm::NewtonCotes => "Newton-Cotes",
        }
    }
}
