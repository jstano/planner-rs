use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::generators::work_generators;
use crate::workcontent::generators::work_generators::{WorkGenerator, WorkGeneratorKind, WorkResults};

pub fn generate_work_content(planner_model: PlannerModel) -> Vec<WorkResults> {
    planner_model
        .jobs()
        .iter()
        .map(|job| {
            let work_generator_kind: WorkGeneratorKind = job.planner_settings().standard_type.into();
            work_generator_kind.generate_work(&planner_model, job)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workcontent::domain::location::{Location, LocationId};
    use crate::workcontent::domain::standard_set::{StandardSet, StandardSetId};
    use date_range_rs::DateRange;
    use std::any::type_name;
    use std::collections::HashMap;
    use joda_rs::LocalDate;
    use crate::workcontent::domain::job::Job;

    #[macro_export]
    macro_rules! date {
        ($y:expr, $m:expr, $d:expr) => {
            LocalDate::new($y, $m, $d)
        };
    }

    #[test]
    fn should_generate_empty_work_content_if_no_jobs() {
        let dates = DateRange::new(
            LocalDate::new(2025, 10, 1),
            LocalDate::new(2025, 10, 31),
        );
        let location = Location::new(LocationId::new());
        let standard_set = StandardSet::new(StandardSetId::new());
        let planner_model = PlannerModel::new(dates, location.id(), standard_set.id(), vec![], vec![], HashMap::new());
        let results = generate_work_content(planner_model);
        assert!(!results.is_empty());
    }

    #[test]
    fn should_generate_empty_work_content_if_one_job() {
        let dates = DateRange::new(
            LocalDate::new(2025, 10, 1),
            LocalDate::new(2025, 10, 31),
        );
        let location = Location::new(LocationId::new());
        let standard_set = StandardSet::new(StandardSetId::new());
        let planner_model = PlannerModel::new(dates, location.id(), standard_set.id(), vec![Job::test()], vec![], HashMap::new());
        let results = generate_work_content(planner_model);
        assert!(!results.is_empty());
    }
}
