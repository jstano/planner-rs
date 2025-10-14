use std::any::Any;
use joda_rs::LocalDate;
use crate::workcontent::domain::job::Job;
use crate::workcontent::domain::job_shift::JobShift;
use crate::workcontent::domain::labor_data::LaborData;
use crate::workcontent::generators::work_generators::{WorkGenerator, WorkResults};
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::generators::salaried::salaried_calculator::SalariedCalculator;

pub struct SalariedWorkGenerator {
    salaried_calculator: SalariedCalculator
}

impl SalariedWorkGenerator {
    pub fn new() -> Self {
        Self {
            salaried_calculator: SalariedCalculator::new()
        }
    }

    pub fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        let mut labor_data_results: Vec<LaborData> = Vec::new();

        for date in job.planner_settings().dates(planner_model) {
            let hours = self.calculate_work_for_all_shifts(planner_model, job, date);

            labor_data_results.push(LaborData::new(job.id(), date, hours));
        }

        WorkResults::with_labor_data(job.id(), labor_data_results)
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

impl WorkGenerator for SalariedWorkGenerator {
    fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        self.generate_work(planner_model, job)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

}
