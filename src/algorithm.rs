use std::ops::RangeInclusive;

use gauss_quad::{GaussChebyshevFirstKind, GaussLegendre};

use crate::{Algorithm, App, Endpoints, InterpolationPoint};

impl App {
    pub fn eval(&self, func: impl Fn(f64) -> f64) -> Vec<InterpolationPoint> {
        match self.selected_algo {
            Algorithm::ClosedNewtonCotes => self.closed_newton_cotes(self.endpoints, func),
            Algorithm::OpenNewtonCotes => self.open_newton_cotes(self.endpoints, func),

            Algorithm::GaussLegendre => self.gauss(
                self.endpoints,
                func,
                GaussLegendre::new(self.gauss_legendre_n.try_into().unwrap())
                    .as_node_weight_pairs(),
            ),
            Algorithm::GaussChebyshev => self.gauss(
                self.endpoints,
                func,
                GaussChebyshevFirstKind::new(self.gauss_legendre_n.try_into().unwrap())
                    .as_node_weight_pairs(),
            ),
        }
    }

    fn closed_newton_cotes(
        &self,
        endpoints: Endpoints,
        func: impl Fn(f64) -> f64,
    ) -> Vec<InterpolationPoint> {
        let (factor, weights) =
            Self::CLOSED_NEWTON_COTES_COEFFICIENTS[self.closed_newton_cotes_n - 1];
        let n = self.closed_newton_cotes_n as f64;

        weights
            .iter()
            .enumerate()
            .map(|(i, &weight)| {
                let i = i as f64;
                let step_h = (endpoints.end - endpoints.start) / n;

                let x = endpoints.start + i * step_h;
                let y = func(x);
                let area = factor * step_h * y * weight;

                InterpolationPoint { x, y, area }
            })
            .collect()
    }

    fn open_newton_cotes(
        &self,
        endpoints: Endpoints,
        func: impl Fn(f64) -> f64,
    ) -> Vec<InterpolationPoint> {
        let (factor, weights) = Self::OPEN_NEWTON_COTES_COEFFICIENTS[self.open_newton_cotes_n];
        let n = self.open_newton_cotes_n as f64;

        weights
            .iter()
            .enumerate()
            .map(|(i, &weight)| {
                let i = i as f64;
                let step_h = (endpoints.end - endpoints.start) / (n + 2.0);

                let x = endpoints.start + (i + 1.0) * step_h;
                let y = func(x);
                let area = factor * step_h * y * weight;

                InterpolationPoint { x, y, area }
            })
            .collect()
    }

    fn gauss(
        &self,
        endpoints: Endpoints,
        func: impl Fn(f64) -> f64,
        pairs: &[(f64, f64)],
    ) -> Vec<InterpolationPoint> {
        let midpoint = (endpoints.end - endpoints.start) / 2.0;
        let offset = (endpoints.end + endpoints.start) / 2.0;

        pairs
            .iter()
            .map(|&(node, weight)| {
                let x = offset + node * midpoint;
                let y = func(x);
                let area = y * weight * midpoint;
                InterpolationPoint { x, y, area }
            })
            .collect()
    }

    pub const CLOSED_NEWTON_COTES_N_RANGE: RangeInclusive<usize> =
        1..=Self::CLOSED_NEWTON_COTES_COEFFICIENTS.len();

    pub const CLOSED_NEWTON_COTES_COEFFICIENTS: &[(f64, &[f64])] = &[
        (1.0 / 2.0, &[1.0, 1.0]),
        (1.0 / 1.3, &[1.0, 4.0, 1.0]),
        (3.0 / 8.0, &[1.0, 3.0, 3.0, 1.0]),
        (2.0 / 45.0, &[7.0, 32.0, 12.0, 32.0, 7.0]),
        (5.0 / 288.0, &[19.0, 75.0, 50.0, 50.0, 75.0, 19.0]),
        (1.0 / 140.0, &[41.0, 216.0, 27.0, 272.0, 27.0, 216.0, 41.0]),
    ];

    pub const OPEN_NEWTON_COTES_N_RANGE: RangeInclusive<usize> =
        0..=(Self::OPEN_NEWTON_COTES_COEFFICIENTS.len() - 1);

    pub const OPEN_NEWTON_COTES_COEFFICIENTS: &[(f64, &[f64])] = &[
        (2.0, &[1.0]),
        (3.0 / 2.0, &[1.0, 1.0]),
        (4.0 / 3.0, &[2.0, -1.0, 2.0]),
        (5.0 / 24.0, &[11.0, 1.0, 1.0, 11.0]),
    ];

    pub const GAUSS_N_RANGE: RangeInclusive<usize> = 1..=10;
}
