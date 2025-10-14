use std::any::Any;
use crate::workcontent::domain::job::Job;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::generators::work_generators::{WorkGenerator, WorkResults};

pub struct NoneWorkGenerator;

impl NoneWorkGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl WorkGenerator for NoneWorkGenerator {
    fn generate_work(&self, _planner_model: &PlannerModel, job: &Job) -> WorkResults {
        WorkResults::with_shifts(job.id(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workcontent::domain::job::Job;
    use crate::workcontent::domain::planner_model::PlannerModel;
    use crate::workcontent::domain::location::LocationId;
    use crate::workcontent::domain::standard_set::StandardSetId;
    use date_range_rs::DateRange;
    use std::collections::HashMap;
    use joda_rs::LocalDate;

    fn make_planner_model() -> PlannerModel {
        let start = LocalDate::new(2025, 1, 1);
        let end = LocalDate::new(2025, 1, 7);
        let dates = DateRange::new(start, end);
        let location_id = LocationId::new();
        let standard_set_id = StandardSetId::new();
        PlannerModel::new(dates, location_id, standard_set_id, vec![], vec![], HashMap::new())
    }

    #[test]
    fn none_generator_returns_empty_shifts_and_correct_job_id() {
        let generator = NoneWorkGenerator::new();
        let planner_model = make_planner_model();
        let job = Job::test();

        let results = generator.generate_work(&planner_model, &job);

        assert_eq!(results.job_id(), job.id());
        let shifts = results.shifts().expect("shifts should be Some for NoneWorkGenerator");
        assert_eq!(shifts.len(), 0);
        assert!(results.labor_data().is_none());
    }

    #[test]
    fn none_generator_ignores_planner_model_content() {
        // Even if planner model has different dates or hypothetical data, output remains empty
        let generator = NoneWorkGenerator::new();
        let planner_model = make_planner_model();
        let job = Job::test();

        let results = generator.generate_work(&planner_model, &job);

        assert!(results.shifts().is_some());
        assert_eq!(results.shifts().unwrap().len(), 0);
    }
}
