mod algorithm;
mod app;
pub mod constants;
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

pub struct QuadOutput {
    pub points: Vec<InterpolationPoint>,
    pub error: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Algorithm {
    ClosedNewtonCotes,
    OpenNewtonCotes,
    GaussLegendre,
    GaussChebyshev,
}

impl Algorithm {
    pub const VARIANTS: &[Algorithm] = &[
        Algorithm::ClosedNewtonCotes,
        Algorithm::OpenNewtonCotes,
        Algorithm::GaussLegendre,
        Algorithm::GaussChebyshev,
    ];

    pub fn text(&self) -> &'static str {
        match self {
            Algorithm::ClosedNewtonCotes => "Closed Newton-Cotes",
            Algorithm::OpenNewtonCotes => "Open Newton-Cotes",
            Algorithm::GaussLegendre => "Gauss-Legendre",
            Algorithm::GaussChebyshev => "Gauss-Chebyshev",
        }
    }
}
