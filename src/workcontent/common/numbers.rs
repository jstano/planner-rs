pub fn round_raw_hours(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn round_hours(value: f64) -> f64 {
    round_raw_hours(value)
}

pub fn truncate(value: f64) -> i32 {
    value.floor() as i32
}

pub fn round_to_i32(value: f64) -> i32 {
    value.round() as i32
}

#[cfg(test)]
mod tests {
    use super::{round_raw_hours, round_hours, truncate, round_to_i32};
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(1.0, 1.0)]
    #[case(1.2, 1.2)] // already at 1 decimal
    #[case(1.23, 1.23)] // already at 2 decimals
    #[case(1.234, 1.23)] // truncate after rounding
    #[case(1.235, 1.24)] // half rounds away from zero
    #[case(1.999, 2.0)] // rounds up to next integer
    #[case(-1.2, -1.2)]
    #[case(-1.234, -1.23)]
    #[case(-1.235, -1.24)] // half away from zero on negatives
    #[case(123456.789, 123456.79)]
    fn test_round_raw_hours(#[case] input: f64, #[case] expected: f64) {
        let actual = round_raw_hours(input);
        let eps = 1e-12;
        assert!(
            (actual - expected).abs() < eps,
            "round_raw_hours({input}) => {actual}, expected {expected}"
        );
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(1.234, 1.23)]
    #[case(1.235, 1.24)]
    #[case(-1.234, -1.23)]
    #[case(-1.235, -1.24)]
    fn test_round_hours(#[case] input: f64, #[case] expected: f64) {
        let actual = round_hours(input);
        let eps = 1e-12;
        assert!(
            (actual - expected).abs() < eps,
            "round_hours({input}) => {actual}, expected {expected}"
        );
    }

    #[rstest]
    #[case(0.0, 0)]
    #[case(0.9, 0)]
    #[case(1.0, 1)]
    #[case(1.999, 1)]
    #[case(-0.001, -1)]
    #[case(-1.0, -1)]
    #[case(-1.001, -2)]
    fn test_truncate(#[case] input: f64, #[case] expected: i32) {
        let actual = truncate(input);
        assert_eq!(actual, expected, "truncate({input}) => {actual}, expected {expected}");
    }

    #[rstest]
    #[case(0.0, 0)]
    #[case(0.49, 0)]
    #[case(0.5, 1)]
    #[case(1.5, 2)]
    #[case(2.49, 2)]
    #[case(2.5, 3)]
    #[case(-0.49, 0)]
    #[case(-0.5, -1)] // ties away from zero
    #[case(-1.5, -2)]
    #[case(-2.5, -3)]
    fn test_round_to_i32(#[case] input: f64, #[case] expected: i32) {
        let actual = round_to_i32(input);
        assert_eq!(actual, expected, "round_to_i32({input}) => {actual}, expected {expected}");
    }
}
