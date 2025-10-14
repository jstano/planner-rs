use joda_rs::{ChronoUnit, DayOfWeek, LocalDate};
use joda_rs::constants::MONTHS_PER_YEAR;
use crate::workcontent::domain::salaried_standard::SalariedStandard;
use crate::workcontent::domain::salary_mode::SalaryMode;

pub struct SalariedCalculator;

impl SalariedCalculator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_hours(&self, standard: &SalariedStandard, date: LocalDate) -> f64 {
        match standard.salary_mode {
            SalaryMode::WEEKLY => monthly_hours(standard, date),
            SalaryMode::MONTHLY => weekly_hours(standard, date),
        }
    }
}

fn monthly_hours(standard: &SalariedStandard, date: LocalDate) -> f64 {
    let hours_per_year = standard.hours_per_year;

    round_hours(hours_per_year /  (MONTHS_PER_YEAR as f64) / (date.length_of_month() as f64))
}

fn round_hours(hours: f64) -> f64 {
    (hours * 100.0).round() / 100.0
}

fn weekly_hours(standard: &SalariedStandard, date: LocalDate) -> f64 {
    let number_not_sunday_days = 6.0;
    let days_per_week = 7.0;
    let weeks_per_year = 52.0;
    let vacation_hours_per_year: f64 = standard.vacation_hours_per_year;

    let std_hours_per_week = standard.hours_per_week;
    let hours_per_year = (std_hours_per_week * weeks_per_year) - vacation_hours_per_year;
    let adj_hours_per_week = hours_per_year / weeks_per_year;

    let mon_sat_hours = round_hours(hours_per_year / weeks_per_year / days_per_week);
    let sun_hours = round_hours(adj_hours_per_week - (mon_sat_hours * number_not_sunday_days));

    if date.day_of_week() == DayOfWeek::Sunday {
        sun_hours
    } else {
        mon_sat_hours
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workcontent::domain::job::{Job, JobId};
    use crate::workcontent::domain::job_shift::JobShift;
    use crate::workcontent::domain::salaried_standard::SalariedStandard;
    use crate::workcontent::domain::salary_mode::SalaryMode;
    use crate::workcontent::domain::standard_set::{StandardSet, StandardSetId};

    // Helper to build a SalariedStandard for WEEKLY mode path (monthly_hours)
    fn std_for_weekly_mode(hours_per_year: f64) -> SalariedStandard {
        // We only use: salary_mode, hours_per_year. Other fields are inert.
        SalariedStandard {
            // SAFETY: These fields are not read by the calculator logic we test.
            job_id: JobId::new(),
            standard_set_id: StandardSetId::new(),
            shift: JobShift::test(),
            salary_mode: SalaryMode::WEEKLY,
            hours_per_week: 0.0,
            vacation_hours_per_year: 0.0,
            hours_per_year,
        }
    }

    // Helper to build a SalariedStandard for MONTHLY mode path (weekly_hours)
    fn std_for_monthly_mode(hours_per_week: f64, vacation_hours_per_year: f64) -> SalariedStandard {
        // We only use: salary_mode, hours_per_week, vacation_hours_per_year. Other fields are inert.
        SalariedStandard {
            job_id: JobId::new(),
            standard_set_id: StandardSetId::new(),
            shift: JobShift::test(),
            salary_mode: SalaryMode::MONTHLY,
            hours_per_week,
            vacation_hours_per_year,
            hours_per_year: 0.0,
        }
    }

    fn d(y: i32, m: i32, d: i32) -> LocalDate {
        LocalDate::new(y, m, d)
    }

    #[test]
    fn weekly_mode_uses_monthly_hours_for_30_day_month() {
        // Choose hours_per_year so that result is exact: 12 * 30 * 4 = 1440 -> 4.0 hours/day
        let std = std_for_weekly_mode(1440.0);
        let calc = SalariedCalculator::new();

        let date = d(2025, 4, 10); // April has 30 days
        let hours = calc.calculate_hours(&std, date);
        assert_eq!(hours, 4.0);
    }

    #[test]
    fn weekly_mode_uses_monthly_hours_for_31_day_month() {
        // 12 * 31 * 4 = 1488 -> 4.0 hours/day
        let std = std_for_weekly_mode(1488.0);
        let calc = SalariedCalculator::new();

        let date = d(2025, 1, 10); // January has 31 days
        let hours = calc.calculate_hours(&std, date);
        assert_eq!(hours, 4.0);
    }

    #[test]
    fn weekly_mode_handles_february_non_leap() {
        // 12 * 28 * 4 = 1344 -> 4.0 hours/day
        let std = std_for_weekly_mode(1344.0);
        let calc = SalariedCalculator::new();

        let date = d(2025, 2, 10); // 2025 is not a leap year (28 days)
        let hours = calc.calculate_hours(&std, date);
        assert_eq!(hours, 4.0);
    }

    #[test]
    fn weekly_mode_handles_february_leap() {
        // 12 * 29 * 4 = 1392 -> 4.0 hours/day
        let std = std_for_weekly_mode(1392.0);
        let calc = SalariedCalculator::new();

        let date = d(2024, 2, 10); // 2024 is a leap year (29 days)
        let hours = calc.calculate_hours(&std, date);
        assert_eq!(hours, 4.0);
    }

    #[test]
    fn monthly_mode_weekly_hours_equal_distribution_example_weekday() {
        // Pick hours_per_week so per-day distribution is exact: 42 hours/week, 0 vacation
        // hours_per_year = 42 * 52 = 2184, mon_sat_hours = (2184 / 52 / 7) = 6.0, sun_hours = 6.0
        let std = std_for_monthly_mode(42.0, 0.0);
        let calc = SalariedCalculator::new();

        let monday = d(2025, 1, 6); // Monday
        assert_eq!(monday.day_of_week(), DayOfWeek::Monday);
        let hours_mon = calc.calculate_hours(&std, monday);
        assert_eq!(hours_mon, 6.0);
    }

    #[test]
    fn monthly_mode_weekly_hours_equal_distribution_example_sunday() {
        // Same setup, Sunday should also be 6.0
        let std = std_for_monthly_mode(42.0, 0.0);
        let calc = SalariedCalculator::new();

        let sunday = d(2025, 1, 5); // Sunday
        assert_eq!(sunday.day_of_week(), DayOfWeek::Sunday);
        let hours_sun = calc.calculate_hours(&std, sunday);
        assert_eq!(hours_sun, 6.0);
    }
}
