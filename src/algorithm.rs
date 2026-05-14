use formulac::{Builder, err::ParseError};
use gauss_quad::{GaussChebyshevFirstKind, GaussLegendre};
use num_complex::Complex;

use crate::{
    Algorithm, App, Endpoints, InterpolationPoint, QuadOutput,
    constants::{CLOSED_NEWTON_COTES_COEFFICIENTS, OPEN_NEWTON_COTES_COEFFICIENTS},
};

impl App {
    pub fn eval(&self) -> Result<QuadOutput, ParseError> {
        match self.selected_algo {
            Algorithm::ClosedNewtonCotes => self.closed_newton_cotes(self.endpoints),
            Algorithm::OpenNewtonCotes => self.open_newton_cotes(self.endpoints),

            Algorithm::GaussLegendre => self.gauss(
                self.endpoints,
                GaussLegendre::new(self.gauss_legendre_n.try_into().unwrap())
                    .as_node_weight_pairs(),
            ),
            Algorithm::GaussChebyshev => self.gauss(
                self.endpoints,
                GaussChebyshevFirstKind::new(self.gauss_legendre_n.try_into().unwrap())
                    .as_node_weight_pairs(),
            ),
        }
    }

    fn closed_newton_cotes(&self, endpoints: Endpoints) -> Result<QuadOutput, ParseError> {
        let func = Builder::<f64, 1>::new(&self.expr, ["x"]).compile()?;

        let entry = CLOSED_NEWTON_COTES_COEFFICIENTS[self.closed_newton_cotes_n - 1];
        let n = self.closed_newton_cotes_n as f64;

        let points = entry
            .weights
            .iter()
            .enumerate()
            .map(|(i, &weight)| {
                let i = i as f64;
                let step_h = (endpoints.end - endpoints.start) / n;

                let x = endpoints.start + i * step_h;
                let y = func([Complex::new(x, 0.0)]).re;
                let area = entry.factor * step_h * y * weight;

                InterpolationPoint { x, y, area }
            })
            .collect();

        Ok(QuadOutput { points, error: 0.0 })
    }

    fn open_newton_cotes(&self, endpoints: Endpoints) -> Result<QuadOutput, ParseError> {
        let func = Builder::<f64, 1>::new(&self.expr, ["x"]).compile()?;

        let entry = OPEN_NEWTON_COTES_COEFFICIENTS[self.open_newton_cotes_n];
        let n = self.open_newton_cotes_n as f64;
        let step_h = (endpoints.end - endpoints.start) / (n + 2.0);

        let points = entry
            .weights
            .iter()
            .enumerate()
            .map(|(node, &weight)| {
                let node = node as f64;

                let x = endpoints.start + (node + 1.0) * step_h;
                let y = func([Complex::new(x, 0.0)]).re;
                let area = entry.factor * step_h * y * weight;

                InterpolationPoint { x, y, area }
            })
            .collect();

        Ok(QuadOutput { points, error: 0.0 })
    }

    fn gauss(&self, endpoints: Endpoints, pairs: &[(f64, f64)]) -> Result<QuadOutput, ParseError> {
        let func = Builder::<f64, 1>::new(&self.expr, ["x"]).compile()?;

        let midpoint = (endpoints.end - endpoints.start) / 2.0;
        let offset = (endpoints.end + endpoints.start) / 2.0;

        let points = pairs
            .iter()
            .map(|&(node, weight)| {
                let x = offset + node * midpoint;
                let y = func([Complex::new(x, 0.0)]).re;
                let area = y * weight * midpoint;
                InterpolationPoint { x, y, area }
            })
            .collect();

        Ok(QuadOutput { points, error: 0.0 })
    }
}
