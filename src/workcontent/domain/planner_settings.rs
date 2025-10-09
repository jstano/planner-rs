use crate::workcontent::domain::meal_break::MealBreak;
use crate::workcontent::domain::non_flowed_distribution_method::NonFlowedDistributionMethod;
use crate::workcontent::domain::non_meal_break::NonMealBreak;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::domain::standard_type::StandardType;
use date_range_rs::DateRange;
use joda_rs::LocalDate;

pub struct PlannerSettings {
    pub standard_type: StandardType,
    pub period_length: u32,
    pub min_shift_length: f64,
    pub max_shift_length: f64,
    pub rounding_threshold_below_one: f64,
    pub rounding_threshold_above_one: f64,
    pub meal_break: Option<MealBreak>,
    pub non_meal_break: Option<NonMealBreak>,
    pub effective_dates: DateRange,
    pub generate_long_shifts: bool,
    pub limit_shift_to_max_shift: bool,
    pub truncate_max_coverage: bool,
    pub non_flowed_distribution_method: NonFlowedDistributionMethod,
}

impl PlannerSettings {
    pub fn default() -> PlannerSettings {
        Self {
            standard_type: StandardType::NONE,
            period_length: 30,
            min_shift_length: 4.0,
            max_shift_length: 8.0,
            rounding_threshold_below_one: 1.0,
            rounding_threshold_above_one: 1.0,
            meal_break: None,
            non_meal_break: None,
            effective_dates: DateRange::new(LocalDate::new(2021, 1, 1), LocalDate::new(2021, 12, 31)),
            generate_long_shifts: false,
            limit_shift_to_max_shift: false,
            truncate_max_coverage: false,
            non_flowed_distribution_method: NonFlowedDistributionMethod::VARYING,
        }
    }

    pub fn dates(&self, planner_model: &PlannerModel) -> Vec<LocalDate> {
        let mut dates: Vec<LocalDate> = Vec::new();

        for date in planner_model.dates().iter() {
            let eff_dates = DateRange::new(
                self.effective_dates.start_date().with_year(date.year()),
                self.effective_dates.end_date().with_year(date.year())
            );

            if eff_dates.contains_date(date) {
                dates.push(date.clone());
            }
        }

        dates
    }
}
