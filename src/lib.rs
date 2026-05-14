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

impl QuadOutput {
    pub fn result(&self) -> f64 {
        self.points.iter().map(|point| point.area).sum()
    }

    pub fn merge(mut self, mut other: Self) -> Self {
        self.points.append(&mut other.points);
        Self {
            points: self.points,
            error: self.error.max(other.error),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Repetition {
    Single,
    Adaptive,
}

impl Repetition {
    pub const VARIANTS: &[Repetition] = &[Repetition::Single, Repetition::Adaptive];

    pub fn text(&self) -> &'static str {
        match self {
            Repetition::Single => "Single",
            Repetition::Adaptive => "Adaptive",
        }
    }
}
