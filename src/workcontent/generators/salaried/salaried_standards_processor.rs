use joda_rs::LocalDate;
use crate::workcontent::domain::job::Job;
use crate::workcontent::domain::job_shift::JobShift;
use crate::workcontent::domain::labor_data::LaborData;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::generators::salaried::salaried_calculator::{
    SalariedCalculator, SalariedCalculatorImpl,
};
use crate::workcontent::generators::work_generators::WorkResults;

pub trait SalariedStandardsProcessor {
    fn process(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults;
}

pub struct SalariedStandardsProcessorImpl {
    salaried_calculator: Box<dyn SalariedCalculator>,
}

impl SalariedStandardsProcessorImpl {
    pub fn new() -> Self {
        Self {
            salaried_calculator: Box::new(SalariedCalculatorImpl::new()),
        }
    }

    fn calculate_work_for_all_shifts(
        &self,
        planner_model: &PlannerModel,
        job: &Job,
        date: LocalDate,
    ) -> f64 {
        job.shifts_for_standard_set(planner_model.standard_set_id())
            .iter()
            .map(|shift| self.calculate_work_for_shift(planner_model, job, shift, date))
            .sum()
    }

    fn calculate_work_for_shift(
        &self,
        planner_model: &PlannerModel,
        job: &Job,
        shift: &JobShift,
        date: LocalDate,
    ) -> f64 {
        let shift_detail = shift.shift_detail_for_date(date);

        if shift_detail.is_some() {
            self.calc_work_for_shift_detail(planner_model, job, shift, date)
        } else {
            0.0
        }
    }

    fn calc_work_for_shift_detail(
        &self,
        planner_model: &PlannerModel,
        job: &Job,
        shift: &JobShift,
        date: LocalDate,
    ) -> f64 {
        let standard =
            job.salaried_standard_for_standard_set_and_shift(planner_model.standard_set_id(), shift);

        if let Some(standard) = standard {
            return self.salaried_calculator.calculate_hours(standard, date);
        }

        0.0
    }
}

impl SalariedStandardsProcessor for SalariedStandardsProcessorImpl {
    fn process(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        let mut labor_data_results: Vec<LaborData> = Vec::new();

        for date in job.planner_settings().dates(planner_model) {
            let hours = self.calculate_work_for_all_shifts(planner_model, job, date);

            labor_data_results.push(LaborData::new(job.id(), date, hours));
        }

        WorkResults::with_labor_data(job.id(), labor_data_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to ensure a type implements the trait
    fn require_impl_trait<T: SalariedStandardsProcessor>() {}

    #[test]
    fn trait_is_object_safe() {
        // If the trait is not object-safe, these won't compile.
        fn _accepts_box(_: Box<dyn SalariedStandardsProcessor>) {}
        fn _accepts_ref(_: &dyn SalariedStandardsProcessor) {}

        // Just reference the functions to avoid dead code warnings
        let _ = _accepts_box as fn(Box<dyn SalariedStandardsProcessor>);
        let _ = _accepts_ref as fn(&dyn SalariedStandardsProcessor);
    }

    #[test]
    fn impl_can_be_constructed() {
        let _impl = SalariedStandardsProcessorImpl::new();
        // Ensure it implements the trait at the type level
        require_impl_trait::<SalariedStandardsProcessorImpl>();
    }

    #[test]
    fn impl_coerces_to_trait_object() {
        let impl_instance = SalariedStandardsProcessorImpl::new();
        let _trait_obj: &dyn SalariedStandardsProcessor = &impl_instance;
        let _boxed_trait: Box<dyn SalariedStandardsProcessor> = Box::new(impl_instance);
        // Successfully boxing and coercion demonstrates object-safety usage.
        let _ = _boxed_trait;
    }
}
