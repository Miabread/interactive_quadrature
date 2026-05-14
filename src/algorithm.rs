use formulac::{Builder, err::ParseError};
use gauss_quad::{GaussChebyshevFirstKind, GaussLegendre};
use num_complex::Complex;

use crate::{
    Algorithm, App, Endpoints, InterpolationPoint, QuadOutput, Repetition,
    constants::{CLOSED_NEWTON_COTES_COEFFICIENTS, OPEN_NEWTON_COTES_COEFFICIENTS},
};

impl App {
    pub fn eval_repetition(&self, endpoints: Endpoints) -> Result<QuadOutput, ParseError> {
        match self.selected_rep {
            Repetition::Single => self.eval_algorithm(endpoints),
            Repetition::Adaptive => self.adaptive(endpoints, self.adaptive_tolerance, 0),
        }
    }

    pub fn adaptive(
        &self,
        endpoints: Endpoints,
        tolerance: f64,
        iteration: usize,
    ) -> Result<QuadOutput, ParseError> {
        let output = self.eval_algorithm(endpoints)?;

        if iteration > 5 || output.error < tolerance {
            return Ok(output);
        }

        let midpoint = (endpoints.start + endpoints.end) / 2.0;

        let left = self.adaptive(
            Endpoints {
                start: endpoints.start,
                end: midpoint,
            },
            tolerance / 2.0,
            iteration + 1,
        )?;

        let right = self.adaptive(
            Endpoints {
                start: midpoint,
                end: endpoints.end,
            },
            tolerance / 2.0,
            iteration + 1,
        )?;

        Ok(left.merge(right))
    }

    pub fn eval_algorithm(&self, endpoints: Endpoints) -> Result<QuadOutput, ParseError> {
        match self.selected_algo {
            Algorithm::ClosedNewtonCotes => self.closed_newton_cotes(endpoints),
            Algorithm::OpenNewtonCotes => self.open_newton_cotes(endpoints),

            Algorithm::GaussLegendre => self.gauss(
                endpoints,
                GaussLegendre::new(self.gauss_legendre_n.try_into().unwrap())
                    .as_node_weight_pairs(),
            ),
            Algorithm::GaussChebyshev => self.gauss(
                endpoints,
                GaussChebyshevFirstKind::new(self.gauss_legendre_n.try_into().unwrap())
                    .as_node_weight_pairs(),
            ),
        }
    }

    fn closed_newton_cotes(&self, endpoints: Endpoints) -> Result<QuadOutput, ParseError> {
        let func = Builder::<f64, 1>::new(&self.expr, ["x"]).compile()?;

        let entry = CLOSED_NEWTON_COTES_COEFFICIENTS[self.closed_newton_cotes_n - 1];
        let n = self.closed_newton_cotes_n as f64;
        let step_h = (endpoints.end - endpoints.start) / n;

        let points = entry
            .weights
            .iter()
            .enumerate()
            .map(|(i, &weight)| {
                let i = i as f64;

                let x = endpoints.start + i * step_h;
                let y = func([Complex::new(x, 0.0)]).re;
                let area = entry.factor * step_h * y * weight;

                InterpolationPoint { x, y, area }
            })
            .collect();

        let error = entry.error_factor
            * step_h.powf(entry.error_exponent)
            * self.error_derivative_term(endpoints, entry.error_diff_order)?;

        Ok(QuadOutput { points, error })
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

        let error = entry.error_factor
            * step_h.powf(entry.error_exponent)
            * self.error_derivative_term(endpoints, entry.error_diff_order)?;

        Ok(QuadOutput { points, error })
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

        let n_i = self.gauss_legendre_n;
        let n_f = n_i as f64;
        let factorial = |n| (1..=n).map(|i| i as f64).product::<f64>();

        let error = ((endpoints.end - endpoints.start).powf(2.0 * n_f + 1.0)
            * factorial(n_i).powf(4.0))
            / ((2.0 * n_f + 1.0) * factorial(2 * n_i).powf(3.0));

        Ok(QuadOutput { points, error })
    }

    fn error_derivative_term(&self, endpoints: Endpoints, order: usize) -> Result<f64, ParseError> {
        let expr = format!("diff({}, x, {})", self.expr, order);
        let func = Builder::<f64, 1>::new(&expr, ["x"]).compile()?;
        let xi = (endpoints.start + endpoints.end) / 2.0;
        Ok(func([Complex::new(xi, 0.0)]).re)
    }
}
