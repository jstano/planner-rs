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
    fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        WorkResults::with_shifts(job.id(), Vec::new())
    }
}
