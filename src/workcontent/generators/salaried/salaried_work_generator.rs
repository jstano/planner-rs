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
    use crate::workcontent::domain::planner_model::PlannerModel;
    use crate::workcontent::domain::location::LocationId;
    use crate::workcontent::domain::standard_set::StandardSetId;
    use crate::workcontent::domain::job::Job;
    use crate::workcontent::domain::planner_settings::PlannerSettings;
    use crate::workcontent::domain::job_shift::JobShift;
    use date_range_rs::DateRange;
    use joda_rs::LocalDate;
    use std::collections::HashMap;

    fn make_planner_model() -> PlannerModel {
        let start = LocalDate::new(2025, 1, 1);
        let end = LocalDate::new(2025, 1, 7);
        let dates = DateRange::new(start, end);
        let location_id = LocationId::new();
        let standard_set_id = StandardSetId::new();
        PlannerModel::new(dates, location_id, standard_set_id, vec![], vec![], HashMap::new())
    }

    #[test]
    fn salaried_generator_returns_labor_data_for_each_date_with_zero_hours_when_no_shifts() {
        let generator = SalariedWorkGenerator::new();
        let planner_model = make_planner_model();
        let job = Job::test();

        let results = generator.generate_work(&planner_model, &job);

        assert_eq!(results.job_id(), job.id());
        assert!(results.shifts().is_none(), "Salaried generator should not return shifts");
        let labor = results.labor_data().expect("labor_data should be Some for SalariedWorkGenerator");

        let expected_days = planner_model.dates().iter().count();
        assert_eq!(labor.len(), expected_days);

        // All hours should be zero because there are no shifts
        assert!(labor.iter().all(|ld| ld.hours() == 0.0));
    }

    #[test]
    fn salaried_generator_sums_zero_hours_when_shifts_have_no_details() {
        let generator = SalariedWorkGenerator::new();
        let mut planner_model = make_planner_model();
        let std_set = planner_model.standard_set_id();

        // Build a couple of shifts under the same standard set but with no definitions => no details on any date
        let shift1 = JobShift::new(
            crate::workcontent::domain::job::JobId::new(),
            std_set,
            "Shift 1".to_string(),
            1,
            vec![],
        );
        let shift2 = JobShift::new(
            crate::workcontent::domain::job::JobId::new(),
            std_set,
            "Shift 2".to_string(),
            2,
            vec![],
        );

        let job = Job::new(
            LocationId::new(),
            PlannerSettings::default(),
            vec![shift1, shift2],
            vec![], // no salaried standards associated
        );

        let results = generator.generate_work(&planner_model, &job);
        let labor = results.labor_data().expect("labor_data should be present");

        // Expect one entry per planner date and all zeros because no shift details exist
        let expected_days = planner_model.dates().iter().count();
        assert_eq!(labor.len(), expected_days);
        assert!(labor.iter().all(|ld| ld.hours() == 0.0));
    }
}
