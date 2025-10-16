use crate::workcontent::common::numbers::round_raw_hours;
use crate::workcontent::domain::environment::Environment;
use crate::workcontent::domain::units::Units;
use crate::workcontent::generators::calculator::work_content_log::WorkContentLogDetail;

const MINUTES_PER_HOUR: f64 = 60.0;

pub struct WorkPerUnitCalculator;

impl WorkPerUnitCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate(
        &self,
        units: Units,
        standard_value: f64,
        business_driver_value: i32,
        shift_length: f64,
    ) -> Option<WorkContentLogDetail> {
        if standard_value == 0.0 {
            return None;
        }

        match units {
            Units::HoursPerUnit => {
                let work_in_minutes = round_raw_hours(
                    standard_value * business_driver_value as f64 * MINUTES_PER_HOUR,
                );
                Some(WorkContentLogDetail::new(
                    work_in_minutes,
                    format!(
                        "{} * {} * {}",
                        trim_f64(standard_value),
                        business_driver_value,
                        MINUTES_PER_HOUR as i32
                    ),
                ))
            }
            Units::MinutesPerUnit => {
                let work_in_minutes = round_raw_hours(standard_value * business_driver_value as f64);
                Some(WorkContentLogDetail::new(
                    work_in_minutes,
                    format!(
                        "{} * {}",
                        trim_f64(standard_value),
                        business_driver_value
                    )
                ))
            }
            Units::UnitsPerHour => {
                let work_in_minutes = round_raw_hours(
                    business_driver_value as f64 / standard_value * MINUTES_PER_HOUR,
                );
                Some(WorkContentLogDetail::new(
                    work_in_minutes,
                    format!(
                        "{} / {} * {}",
                        business_driver_value,
                        trim_f64(standard_value),
                        MINUTES_PER_HOUR as i32
                    )
                ))
            }
            Units::UnitsPerMinute => {
                let work_in_minutes = round_raw_hours(business_driver_value as f64 / standard_value);
                Some(WorkContentLogDetail::new(
                    work_in_minutes,
                    format!(
                        "{} / {}",
                        business_driver_value,
                        trim_f64(standard_value)
                    )
                ))
            }
            Units::Hours => {
                let work_in_minutes = round_raw_hours(standard_value * MINUTES_PER_HOUR);
                Some(WorkContentLogDetail::new(
                    work_in_minutes,
                    format!(
                        "{} * {}",
                        trim_f64(standard_value),
                        MINUTES_PER_HOUR as i32
                    )
                ))
            }
            Units::Minutes => {
                let work_in_minutes = round_raw_hours(standard_value);
                Some(WorkContentLogDetail::new(
                    work_in_minutes,
                    format!("{}", trim_f64(standard_value))
                ))
            }
            Units::UnitsPerShift => {
                let work_in_minutes = round_raw_hours(
                    business_driver_value as f64 / standard_value * shift_length * MINUTES_PER_HOUR,
                );
                Some(WorkContentLogDetail::new(
                    work_in_minutes,
                    format!(
                        "{} / {} * {} * {}",
                        business_driver_value,
                        trim_f64(standard_value),
                        trim_f64(shift_length),
                        MINUTES_PER_HOUR as i32
                    )
                ))
            }
        }
    }
}

fn trim_f64(v: f64) -> String {
    // Formats f64 similar to Java's default toString without trailing zeros when possible
    // Keep reasonable precision to avoid scientific notation for typical minute values.
    let s = format!("{}", v);
    if s.contains('.') {
        let s = s.trim_end_matches('0').trim_end_matches('.').to_string();
        if s.is_empty() { "0".to_string() } else { s }
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hours_per_unit_calculates_minutes_and_formula() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::HoursPerUnit, 1.5, 10, 8.0).unwrap();
        assert_eq!(result.work_in_minutes(), 900.0);
        assert_eq!(result.formula(), "1.5 * 10 * 60");
    }

    #[test]
    fn minutes_per_unit_calculates_minutes_and_formula() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::MinutesPerUnit, 2.25, 4, 8.0).unwrap();
        assert_eq!(result.work_in_minutes(), 9.0);
        assert_eq!(result.formula(), "2.25 * 4");
    }

    #[test]
    fn units_per_hour_calculates_minutes_and_formula_with_trimming() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::UnitsPerHour, 5.0, 20, 8.0).unwrap();
        assert_eq!(result.work_in_minutes(), 240.0);
        assert_eq!(result.formula(), "20 / 5 * 60"); // 5.0 trimmed to 5
    }

    #[test]
    fn units_per_minute_calculates_minutes_and_formula() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::UnitsPerMinute, 2.5, 15, 8.0).unwrap();
        assert_eq!(result.work_in_minutes(), 6.0);
        assert_eq!(result.formula(), "15 / 2.5");
    }

    #[test]
    fn hours_calculates_minutes_and_formula() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::Hours, 3.0, 0, 0.0).unwrap();
        assert_eq!(result.work_in_minutes(), 180.0);
        assert_eq!(result.formula(), "3 * 60");
    }

    #[test]
    fn minutes_calculates_minutes_and_formula() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::Minutes, 17.5, 0, 0.0).unwrap();
        assert_eq!(result.work_in_minutes(), 17.5);
        assert_eq!(result.formula(), "17.5");
    }

    #[test]
    fn units_per_shift_calculates_minutes_and_formula() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::UnitsPerShift, 10.0, 100, 7.5).unwrap();
        assert_eq!(result.work_in_minutes(), 4500.0);
        assert_eq!(result.formula(), "100 / 10 * 7.5 * 60");
    }

    #[test]
    fn zero_standard_value_returns_none() {
        let calc = WorkPerUnitCalculator::new();
        let result = calc.calculate(Units::MinutesPerUnit, 0.0, 10, 8.0);
        assert!(result.is_none());
    }

    #[test]
    fn rounding_to_two_decimals_is_applied() {
        let calc = WorkPerUnitCalculator::new();
        // 1.237 * 3 = 3.711 -> rounds to 3.71
        let result = calc.calculate(Units::MinutesPerUnit, 1.237, 3, 0.0).unwrap();
        assert_eq!(result.work_in_minutes(), 3.71);
    }
}
