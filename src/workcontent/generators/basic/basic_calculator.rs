use crate::workcontent::common::numbers::{round_hours, round_raw_hours, round_to_i32, truncate};
use crate::workcontent::domain::meal_break::MealBreak;
use crate::workcontent::domain::non_meal_break::NonMealBreak;
use crate::workcontent::domain::planner_settings::PlannerSettings;

const MINUTES_PER_HOUR: f64 = 60.0;

pub struct BasicCalculationResult {
    pub number_of_full_time_shifts: i32,
    pub remaining_work_hours: f64,
    pub work_hours_to_cover_breaks: f64,
}

pub struct BasicCalculator;

impl BasicCalculator {
    pub fn new() -> Self { Self }

    /// total_work_minutes: total minutes of work to cover (before adding paid breaks)
    /// shift_length: configured shift length for the assignment (in hours)
    pub fn calculate(&self, planner_settings: &PlannerSettings, shift_length: f64, total_work_minutes: i32) -> BasicCalculationResult {
        let shift_length = self.get_shift_length(planner_settings, shift_length);
        let productive_full_shift_hours = Self::calculate_productive_time_from_total_duration(
            shift_length,
            planner_settings.meal_break.as_ref(),
            planner_settings.non_meal_break.as_ref(),
        );
        let break_hours_per_full_shift = shift_length - productive_full_shift_hours;

        let total_work_hours = round_hours(total_work_minutes as f64 / MINUTES_PER_HOUR);
        let full_time_shifts = truncate((total_work_hours / productive_full_shift_hours) as f64);
        let paid_breaks_for_full_time_shifts = full_time_shifts as f64 * break_hours_per_full_shift;

        let remaining_work_hours = self.get_remaining_work_hours(
            planner_settings,
            productive_full_shift_hours,
            total_work_hours,
            full_time_shifts,
            shift_length,
        );
        let paid_breaks_for_remaining_work = if remaining_work_hours == 0.0 {
            0.0
        } else {
            Self::calculate_break_in_fractional_hours(
                remaining_work_hours,
                planner_settings.meal_break.as_ref(),
                planner_settings.non_meal_break.as_ref(),
            )
        };

        BasicCalculationResult {
            number_of_full_time_shifts: full_time_shifts,
            remaining_work_hours: remaining_work_hours + paid_breaks_for_remaining_work,
            work_hours_to_cover_breaks: paid_breaks_for_full_time_shifts + paid_breaks_for_remaining_work,
        }
    }

    fn get_shift_length(&self, planner_settings: &PlannerSettings, assignment_shift_length: f64) -> f64 {
        if planner_settings.max_shift_length > 0.0 && planner_settings.max_shift_length < assignment_shift_length {
            planner_settings.max_shift_length
        } else {
            assignment_shift_length
        }
    }

    fn get_remaining_work_hours(&self,
                                planner_settings: &PlannerSettings,
                                productive_full_shift_hours: f64,
                                total_work_hours: f64,
                                full_time_shifts: i32,
                                assignment_shift_length: f64) -> f64 {
        let min_shift = planner_settings.min_shift_length;
        let mut remaining_work_hours = round_hours(total_work_hours % productive_full_shift_hours);
        if round_to_i32(remaining_work_hours * MINUTES_PER_HOUR) < self.calculate_rounding_threshold_minutes(full_time_shifts, planner_settings) {
            remaining_work_hours = 0.0;
        } else if remaining_work_hours < min_shift {
            let shift_length = assignment_shift_length;
            remaining_work_hours = if shift_length < min_shift { shift_length } else { min_shift };
        }

        self.round_remaining_hours_to_nearest_period(remaining_work_hours, planner_settings)
    }

    fn round_remaining_hours_to_nearest_period(&self, remaining_hours: f64, planner_settings: &PlannerSettings) -> f64 {
        let remaining_work_minutes = round_to_i32(remaining_hours * MINUTES_PER_HOUR);
        let period_length = planner_settings.period_length as i32;
        if remaining_work_minutes > 0 && remaining_work_minutes < period_length {
            return round_hours(period_length as f64 / MINUTES_PER_HOUR);
        }

        let full_periods = round_to_i32(remaining_work_minutes as f64 / period_length as f64);
        round_hours(full_periods as f64 * period_length as f64 / MINUTES_PER_HOUR)
    }

    fn calculate_rounding_threshold_minutes(&self, full_time_shifts: i32, planner_settings: &PlannerSettings) -> i32 {
        let round_threshold = if full_time_shifts < 1 { planner_settings.rounding_threshold_below_one } else { planner_settings.rounding_threshold_above_one };
        truncate(planner_settings.period_length as f64 * round_threshold)
    }

    // ---------------- Break calculations (ported from Java BreakLengthCalculator) ----------------

    fn calculate_break_in_fractional_hours(shift_length: f64, meal_break: Option<&MealBreak>, non_meal_break: Option<&NonMealBreak>) -> f64 {
        if meal_break.is_none() || non_meal_break.is_none() {
            return 0.0;
        }
        let meal = meal_break.unwrap();
        let non_meal = non_meal_break.unwrap();

        if meal.break_length == 0.0 && non_meal.break_length == 0.0 {
            return 0.0;
        }

        // If only one type is configured
        if meal.break_length == 0.0 {
            return Self::calculate_break_given_only_non_meal_break(shift_length, non_meal);
        }
        if non_meal.break_length == 0.0 {
            return Self::calculate_break_given_only_meal_break(shift_length, meal);
        }

        Self::calculate_break_given_both_break_types(shift_length, meal, non_meal)
    }

    fn calculate_productive_time_from_total_duration(shift_length: f64, meal_break: Option<&MealBreak>, non_meal_break: Option<&NonMealBreak>) -> f64 {
        if meal_break.is_none() || non_meal_break.is_none() {
            return shift_length;
        }
        let meal = meal_break.unwrap();
        let non_meal = non_meal_break.unwrap();

        if meal.break_length == 0.0 && non_meal.break_length == 0.0 {
            return shift_length;
        }

        if non_meal.break_length == 0.0 {
            return Self::calculate_productive_time_given_meal_break_only(shift_length, meal);
        }
        if meal.break_length == 0.0 {
            return Self::calculate_productive_time_given_non_meal_break_only(shift_length, non_meal);
        }

        Self::calculate_productive_time_given_both_break_types(shift_length, meal, non_meal)
    }

    fn calculate_break_given_both_break_types(shift_length: f64, meal_break: &MealBreak, non_meal_break: &NonMealBreak) -> f64 {
        if shift_length < non_meal_break.break_every { return 0.0; }
        if shift_length < meal_break.break_after { return non_meal_break.break_length; }
        if shift_length < Self::first_multiple_of_non_meal_break_after_meal_break(meal_break, non_meal_break) {
            return non_meal_break.break_length + meal_break.break_length;
        }
        non_meal_break.break_length * 2.0 + meal_break.break_length
    }

    fn calculate_break_given_only_meal_break(shift_length: f64, meal_break: &MealBreak) -> f64 {
        if shift_length < meal_break.break_after { 0.0 } else { meal_break.break_length }
    }

    fn calculate_break_given_only_non_meal_break(shift_length: f64, non_meal_break: &NonMealBreak) -> f64 {
        if shift_length < non_meal_break.break_every { return 0.0; }
        let mut number_of_breaks = 1.0;
        let remaining_shift = shift_length - non_meal_break.break_every;
        number_of_breaks += truncate(remaining_shift / (non_meal_break.break_every + non_meal_break.break_length)) as f64;
        number_of_breaks * non_meal_break.break_length
    }

    fn calculate_productive_time_given_both_break_types(shift_length: f64, meal_break: &MealBreak, non_meal_break: &NonMealBreak) -> f64 {
        let mut productive_time = shift_length;
        if productive_time >= non_meal_break.break_every + non_meal_break.break_length {
            productive_time -= non_meal_break.break_length;
        }
        if productive_time >= meal_break.break_after + meal_break.break_length {
            productive_time -= meal_break.break_length;
        }
        if productive_time >= Self::first_multiple_of_non_meal_break_after_meal_break(meal_break, non_meal_break) + non_meal_break.break_length {
            productive_time -= non_meal_break.break_length;
        }
        productive_time
    }

    fn calculate_productive_time_given_meal_break_only(shift_length: f64, meal_break: &MealBreak) -> f64 {
        if shift_length >= meal_break.break_after + meal_break.break_length { shift_length - meal_break.break_length } else { shift_length }
    }

    fn calculate_productive_time_given_non_meal_break_only(shift_length: f64, non_meal_break: &NonMealBreak) -> f64 {
        let total_number_of_breaks = truncate(shift_length / (non_meal_break.break_every + non_meal_break.break_length)) as f64;
        shift_length - total_number_of_breaks * non_meal_break.break_length
    }

    fn first_multiple_of_non_meal_break_after_meal_break(meal_break: &MealBreak, non_meal_break: &NonMealBreak) -> f64 {
        let mut multiple = 0.0;
        while multiple <= meal_break.break_after {
            multiple += non_meal_break.break_every;
        }
        multiple
    }
}


#[cfg(test)]
mod tests {
    use super::BasicCalculator;
    use super::MINUTES_PER_HOUR;
    use crate::workcontent::domain::planner_settings::PlannerSettings;
    use crate::workcontent::domain::meal_break::MealBreak;
    use crate::workcontent::domain::non_meal_break::NonMealBreak;

    fn mk_settings() -> PlannerSettings {
        PlannerSettings::default()
    }

    #[test]
    fn no_breaks_exact_full_shift() {
        let calc = BasicCalculator::new();
        let mut settings = mk_settings();
        // No breaks configured
        settings.meal_break = None;
        settings.non_meal_break = None;
        settings.period_length = 30; // minutes
        settings.rounding_threshold_below_one = 1.0;
        settings.rounding_threshold_above_one = 1.0;
        settings.min_shift_length = 4.0;
        settings.max_shift_length = 8.0;

        let shift_length = 8.0; // hours
        let total_work_minutes = (8.0 * MINUTES_PER_HOUR) as i32;

        let res = calc.calculate(&settings, shift_length, total_work_minutes);
        assert_eq!(res.number_of_full_time_shifts, 1);
        assert!((res.remaining_work_hours - 0.0).abs() < 1e-12);
        assert!((res.work_hours_to_cover_breaks - 0.0).abs() < 1e-12);
    }

    #[test]
    fn both_breaks_full_productive_shift() {
        let calc = BasicCalculator::new();
        let mut settings = mk_settings();
        settings.meal_break = Some(MealBreak { break_after: 5.0, break_length: 0.5 });
        settings.non_meal_break = Some(NonMealBreak { break_every: 4.0, break_length: 0.25 });
        settings.period_length = 30;
        settings.rounding_threshold_below_one = 1.0;
        settings.rounding_threshold_above_one = 1.0;
        settings.min_shift_length = 4.0;
        settings.max_shift_length = 8.0;

        // Productive hours per full 8h shift = 8 - 0.25 (rest) - 0.5 (meal) = 7.25
        let productive_per_shift = 7.25;
        let total_work_minutes = (productive_per_shift * MINUTES_PER_HOUR) as i32; // 435
        let res = calc.calculate(&settings, 8.0, total_work_minutes);

        assert_eq!(res.number_of_full_time_shifts, 1);
        // remaining work (productive) is zero → remaining_work_hours field includes only remaining's paid breaks (none)
        assert!((res.remaining_work_hours - 0.0).abs() < 1e-12);
        // break hours to cover for the full shift: 0.75
        assert!((res.work_hours_to_cover_breaks - 0.75).abs() < 1e-12);
    }

    #[test]
    fn remainder_below_threshold_zeroed() {
        let calc = BasicCalculator::new();
        let mut settings = mk_settings();
        settings.meal_break = None;
        settings.non_meal_break = None;
        settings.period_length = 30; // minutes
        settings.rounding_threshold_above_one = 1.0; // threshold = 30 minutes
        settings.rounding_threshold_below_one = 1.0;

        // One full 8h shift plus 10 minutes remainder
        let shift_length = 8.0;
        let total_work_minutes = (8.0 * MINUTES_PER_HOUR + 10.0) as i32; // 490
        let res = calc.calculate(&settings, shift_length, total_work_minutes);

        assert_eq!(res.number_of_full_time_shifts, 1);
        // remainder 10 minutes < threshold 30 → becomes 0
        assert!((res.remaining_work_hours - 0.0).abs() < 1e-12);
        assert!((res.work_hours_to_cover_breaks - 0.0).abs() < 1e-12);
    }

    #[test]
    fn round_remaining_to_nearest_period_under_one_shift_threshold_zero() {
        let calc = BasicCalculator::new();
        let mut settings = mk_settings();
        settings.meal_break = None;
        settings.non_meal_break = None;
        settings.period_length = 30; // minutes
        settings.rounding_threshold_below_one = 0.0; // do not zero-out small remainders when <1 full shift
        settings.rounding_threshold_above_one = 1.0;
        settings.min_shift_length = 0.0; // allow small remainders without bumping to min shift

        // Total 100 minutes (< one full shift). It should round to nearest 30-minute multiple → 90 minutes = 1.5 hours
        let shift_length = 8.0;
        let total_work_minutes = 100;
        let res = calc.calculate(&settings, shift_length, total_work_minutes);

        assert_eq!(res.number_of_full_time_shifts, 0);
        assert!((res.remaining_work_hours - 1.5).abs() < 1e-12);
        assert!((res.work_hours_to_cover_breaks - 0.0).abs() < 1e-12);
    }

    #[test]
    fn paid_breaks_added_for_remaining_fractional_shift() {
        let calc = BasicCalculator::new();
        let mut settings = mk_settings();
        // Configure a non-meal break every 2h for 0.25h; meal after 5h for 0.5h (won't apply for 2.5h)
        settings.meal_break = Some(MealBreak { break_after: 5.0, break_length: 0.5 });
        settings.non_meal_break = Some(NonMealBreak { break_every: 2.0, break_length: 0.25 });
        settings.period_length = 30;
        settings.rounding_threshold_below_one = 0.0;
        settings.rounding_threshold_above_one = 1.0;
        settings.min_shift_length = 0.0; // allow small remainders without bumping to min shift

        // Total 2.5h of productive work (< one full shift). Non-meal break should add 0.25h.
        let shift_length = 8.0;
        let total_work_minutes = (2.5 * MINUTES_PER_HOUR) as i32; // 150
        let res = calc.calculate(&settings, shift_length, total_work_minutes);

        assert_eq!(res.number_of_full_time_shifts, 0);
        // remaining_work_hours includes paid breaks for remaining work: 2.5 + 0.25 = 2.75
        assert!((res.remaining_work_hours - 2.75).abs() < 1e-12);
        assert!((res.work_hours_to_cover_breaks - 0.25).abs() < 1e-12);
    }
}
