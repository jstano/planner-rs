use std::any::Any;
use crate::workcontent::domain::job::Job;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::generators::work_generators::{WorkGenerator, WorkResults};
use crate::workcontent::generators::basic::basic_standards_processor::{BasicStandardsProcessor, BasicStandardsProcessorImpl};

pub struct BasicWorkGenerator {
    processor: Box<dyn BasicStandardsProcessor>,
}

impl BasicWorkGenerator {
    pub fn new() -> Self {
        Self { processor: Box::new(BasicStandardsProcessorImpl::new()) }
    }
}

impl WorkGenerator for BasicWorkGenerator{
    fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        self.processor.process(planner_model, job)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
