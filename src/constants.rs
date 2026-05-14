use std::ops::RangeInclusive;

pub const CLOSED_NEWTON_COTES_N_RANGE: RangeInclusive<usize> =
    1..=CLOSED_NEWTON_COTES_COEFFICIENTS.len();

#[derive(Debug, Clone, Copy)]
pub struct NewtonCotesEntry {
    pub factor: f64,
    pub weights: &'static [f64],
    pub error_factor: f64,
    pub error_exponent: f64,
    pub error_diff_order: usize,
}

pub const CLOSED_NEWTON_COTES_COEFFICIENTS: &[NewtonCotesEntry] = &[
    NewtonCotesEntry {
        factor: 1.0 / 2.0,
        weights: &[1.0, 1.0],
        error_factor: -1.0 / 12.0,
        error_exponent: 3.0,
        error_diff_order: 2,
    },
    NewtonCotesEntry {
        factor: 1.0 / 1.3,
        weights: &[1.0, 4.0, 1.0],
        error_factor: -1.0 / 90.0,
        error_exponent: 5.0,
        error_diff_order: 4,
    },
    NewtonCotesEntry {
        factor: 3.0 / 8.0,
        weights: &[1.0, 3.0, 3.0, 1.0],
        error_factor: -3.0 / 80.0,
        error_exponent: 5.0,
        error_diff_order: 4,
    },
    NewtonCotesEntry {
        factor: 2.0 / 45.0,
        weights: &[7.0, 32.0, 12.0, 32.0, 7.0],
        error_factor: -8.0 / 945.0,
        error_exponent: 7.0,
        error_diff_order: 6,
    },
    NewtonCotesEntry {
        factor: 5.0 / 288.0,
        weights: &[19.0, 75.0, 50.0, 50.0, 75.0, 19.0],
        error_factor: -275.0 / 12096.0,
        error_exponent: 7.0,
        error_diff_order: 6,
    },
    NewtonCotesEntry {
        factor: 1.0 / 140.0,
        weights: &[41.0, 216.0, 27.0, 272.0, 27.0, 216.0, 41.0],
        error_factor: -9.0 / 1400.0,
        error_exponent: 9.0,
        error_diff_order: 8,
    },
];

pub const OPEN_NEWTON_COTES_N_RANGE: RangeInclusive<usize> =
    0..=(OPEN_NEWTON_COTES_COEFFICIENTS.len() - 1);

pub const OPEN_NEWTON_COTES_COEFFICIENTS: &[NewtonCotesEntry] = &[
    NewtonCotesEntry {
        factor: 2.0,
        weights: &[1.0],
        error_factor: 1.0 / 3.0,
        error_exponent: 3.0,
        error_diff_order: 2,
    },
    NewtonCotesEntry {
        factor: 3.0 / 2.0,
        weights: &[1.0, 1.0],
        error_factor: 3.0 / 4.0,
        error_exponent: 3.0,
        error_diff_order: 2,
    },
    NewtonCotesEntry {
        factor: 4.0 / 3.0,
        weights: &[2.0, -1.0, 2.0],
        error_factor: 14.0 / 45.0,
        error_exponent: 5.0,
        error_diff_order: 4,
    },
    NewtonCotesEntry {
        factor: 5.0 / 24.0,
        weights: &[11.0, 1.0, 1.0, 11.0],
        error_factor: 95.0 / 144.0,
        error_exponent: 5.0,
        error_diff_order: 4,
    },
];

pub const GAUSS_N_RANGE: RangeInclusive<usize> = 1..=10;
