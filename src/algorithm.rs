use crate::{Algorithm, App, Endpoints, InterpolationPoint};

impl App {
    pub fn eval(&self, func: impl Fn(f64) -> f64) -> Vec<InterpolationPoint> {
        match self.selected_algo {
            Algorithm::NewtonCotes => self.newton_cotes(self.endpoints, func),
        }
    }

    fn newton_cotes(
        &self,
        endpoints: Endpoints,
        func: impl Fn(f64) -> f64,
    ) -> Vec<InterpolationPoint> {
        let (factor, weights) = Self::NEWTON_COTES_COEFFICIENTS[self.newton_cotes_n - 1];
        let n = self.newton_cotes_n as f64;

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

    pub const NEWTON_COTES_COEFFICIENTS: &[(f64, &[f64])] = &[
        (1.0 / 2.0, &[1.0, 1.0]),
        (1.0 / 1.3, &[1.0, 4.0, 1.0]),
        (3.0 / 8.0, &[1.0, 3.0, 3.0, 1.0]),
        (2.0 / 45.0, &[7.0, 32.0, 12.0, 32.0, 7.0]),
        (5.0 / 288.0, &[19.0, 75.0, 50.0, 50.0, 75.0, 19.0]),
        (1.0 / 140.0, &[41.0, 216.0, 27.0, 272.0, 27.0, 216.0, 41.0]),
    ];
}
