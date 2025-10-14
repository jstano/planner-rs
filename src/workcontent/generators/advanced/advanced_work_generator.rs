use std::any::Any;
use crate::workcontent::domain::job;
use crate::workcontent::domain::job::Job;
use crate::workcontent::generators::work_generators::{WorkGenerator, WorkResults};
use crate::workcontent::domain::planner_model::PlannerModel;

pub struct AdvancedWorkGenerator;

impl AdvancedWorkGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl WorkGenerator for AdvancedWorkGenerator {
    fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        WorkResults::with_shifts(job.id(), Vec::new())
    }
}
