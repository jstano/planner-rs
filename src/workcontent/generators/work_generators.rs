use std::any::Any;
use crate::workcontent::domain::job::{Job, JobId};
use crate::workcontent::domain::labor_data::LaborData;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::domain::standard_type::StandardType;
use crate::workcontent::generators::advanced::advanced::AdvancedWorkGenerator;
use crate::workcontent::generators::basic::basic::BasicWorkGenerator;
use crate::workcontent::generators::none::none::NoneWorkGenerator;
use crate::workcontent::generators::salaried::salaried_work_generator::SalariedWorkGenerator;

pub struct WorkResults {
    job_id: JobId,
    shifts: Option<Vec<String>>,
    labor_data: Option<Vec<LaborData>>,
}

impl WorkResults {
    pub fn with_shifts(job_id: JobId, shifts: Vec<String>) -> Self {
        Self {
            job_id,
            shifts: Some(shifts),
            labor_data: None,
        }
    }

    pub fn with_labor_data(job_id: JobId, labor_data: Vec<LaborData>) -> Self {
        Self {
            job_id,
            shifts: None,
            labor_data: Some(labor_data),
        }
    }

    pub fn job_id(&self) -> JobId {
        self.job_id
    }

    pub fn shifts(&self) -> Option<&Vec<String>> {
        self.shifts.as_ref()
    }

    pub fn labor_data(&self) -> Option<&Vec<LaborData>> {
        self.labor_data.as_ref()
    }
}

pub trait WorkGenerator: Any {
    fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults;

    fn as_any(&self) -> &dyn Any;
}

pub fn create(standard_type: StandardType) -> Box<dyn WorkGenerator> {
    match standard_type {
        StandardType::NONE => Box::new(NoneWorkGenerator::new()),
        StandardType::BASIC => Box::new(BasicWorkGenerator::new()),
        StandardType::ADVANCED => Box::new(AdvancedWorkGenerator::new()),
        StandardType::SALARIED => Box::new(SalariedWorkGenerator::new()),
    }
}

#[cfg(test)]
mod tests {
    use crate::workcontent::domain::standard_type::StandardType;
    use crate::workcontent::generators::advanced::advanced::AdvancedWorkGenerator;
    use crate::workcontent::generators::basic::basic::BasicWorkGenerator;
    use crate::workcontent::generators::none::none::NoneWorkGenerator;
    use crate::workcontent::generators::salaried::salaried_work_generator::SalariedWorkGenerator;
    use crate::workcontent::generators::work_generators::{create, WorkResults};
    use rstest::rstest;
    use std::any::{Any, TypeId};
    use joda_rs::LocalDate;
    use crate::workcontent::domain::job::JobId;
    use crate::workcontent::domain::labor_data::LaborData;

    #[test]
    fn should_be_able_to_create_work_results_with_shifts() {
        let job_id = JobId::new();
        let work_results = WorkResults::with_shifts(job_id, vec!["shift1".to_string()]);

        assert_eq!(work_results.job_id(), job_id);
        assert_eq!(work_results.shifts().unwrap().len(), 1);
        assert_eq!(work_results.shifts().unwrap()[0], "shift1");
        assert!(work_results.labor_data().is_none());
    }

    #[test]
    fn should_be_able_to_create_work_results_with_labor_data() {
        let job_id = JobId::new();
        let labor_data = LaborData::new(JobId::new(), LocalDate::new(2025, 10, 6), 123.45);
        let work_results = WorkResults::with_labor_data(job_id, vec![labor_data.clone()]);

        assert_eq!(work_results.job_id(), job_id);
        assert_eq!(work_results.labor_data().unwrap().len(), 1);
        assert_eq!(work_results.labor_data().unwrap()[0], labor_data);
        assert!(work_results.shifts().is_none());
    }

    #[rstest]
    #[case(StandardType::NONE, |g: &dyn Any| g.is::<NoneWorkGenerator>())]
    #[case(StandardType::BASIC, |g: &dyn Any| g.is::<BasicWorkGenerator>())]
    #[case(StandardType::ADVANCED, |g: &dyn Any| g.is::<AdvancedWorkGenerator>())]
    #[case(StandardType::SALARIED, |g: &dyn Any| g.is::<SalariedWorkGenerator>())]
    fn create_should_return_the_correct_generator(
        #[case] standard_type: StandardType,
        #[case] type_check: fn(&dyn Any) -> bool,
    ) {
        let generator = create(standard_type);

        assert!(type_check(generator.as_any()));
    }
}
