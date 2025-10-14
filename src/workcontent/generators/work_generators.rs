use std::any::Any;
use crate::workcontent::domain::job::{Job, JobId};
use crate::workcontent::domain::labor_data::LaborData;
use crate::workcontent::domain::planned_shift::PlannedShift;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::domain::salaried_standard::SalariedStandard;
use crate::workcontent::domain::standard_type::StandardType;
use crate::workcontent::generators::advanced::advanced_work_generator::AdvancedWorkGenerator;
use crate::workcontent::generators::basic::basic_work_generator::BasicWorkGenerator;
use crate::workcontent::generators::none::none_work_generator::NoneWorkGenerator;
use crate::workcontent::generators::salaried::salaried_work_generator::SalariedWorkGenerator;

pub struct WorkResults {
    job_id: JobId,
    shifts: Option<Vec<PlannedShift>>,
    labor_data: Option<Vec<LaborData>>,
}

impl WorkResults {
    pub fn with_shifts(job_id: JobId, shifts: Vec<PlannedShift>) -> Self {
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

    pub fn shifts(&self) -> Option<&Vec<PlannedShift>> {
        self.shifts.as_ref()
    }

    pub fn labor_data(&self) -> Option<&Vec<LaborData>> {
        self.labor_data.as_ref()
    }
}

pub trait WorkGenerator {
    fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults;
}

pub enum WorkGeneratorKind {
    None(NoneWorkGenerator),
    Basic(BasicWorkGenerator),
    Advanced(AdvancedWorkGenerator),
    Salaried(SalariedWorkGenerator),
}

impl WorkGeneratorKind {
    pub fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        match self {
            WorkGeneratorKind::None(generator) => generator.generate_work(planner_model, job),
            WorkGeneratorKind::Basic(generator) => generator.generate_work(planner_model, job),
            WorkGeneratorKind::Advanced(generator) => generator.generate_work(planner_model, job),
            WorkGeneratorKind::Salaried(generator) => generator.generate_work(planner_model, job),
        }
    }
}

impl From<StandardType> for WorkGeneratorKind {
    fn from(standard_type: StandardType) -> Self {
        match standard_type {
            StandardType::NONE => WorkGeneratorKind::None(NoneWorkGenerator::new()),
            StandardType::BASIC => WorkGeneratorKind::Basic(BasicWorkGenerator::new()),
            StandardType::ADVANCED => WorkGeneratorKind::Advanced(AdvancedWorkGenerator::new()),
            StandardType::SALARIED => WorkGeneratorKind::Salaried(SalariedWorkGenerator::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workcontent::domain::standard_type::StandardType;
    use crate::workcontent::generators::advanced::advanced_work_generator::AdvancedWorkGenerator;
    use crate::workcontent::generators::none::none_work_generator::NoneWorkGenerator;
    use crate::workcontent::generators::salaried::salaried_work_generator::SalariedWorkGenerator;
    use crate::workcontent::generators::work_generators::{WorkGeneratorKind, WorkResults};
    use rstest::rstest;
    use std::any::{Any, TypeId};
    use joda_rs::LocalDate;
    use crate::workcontent::domain::job::JobId;
    use crate::workcontent::domain::labor_data::LaborData;

    #[test]
    fn should_be_able_to_create_work_results_with_shifts() {
        let job_id = JobId::new();
        let work_results = WorkResults::with_shifts(job_id, vec![]);

        assert_eq!(work_results.job_id(), job_id);
        assert_eq!(work_results.shifts().unwrap().len(), 1);
        // assert_eq!(work_results.shifts().unwrap()[0], "shift1");
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
}
