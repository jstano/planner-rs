use joda_rs::LocalDate;
use crate::workcontent::domain::job::Job;
use crate::workcontent::domain::job_shift::JobShift;
use crate::workcontent::domain::planned_shift::PlannedShift;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::domain::standard_type::StandardType::BASIC;
use crate::workcontent::generators::work_generators::{WorkGenerator, WorkResults};

pub struct BasicWorkGenerator;

impl BasicWorkGenerator {
    pub fn new() -> Self {
        Self {}
    }

    fn has_shift_on_date(shift: &JobShift, date: LocalDate) -> bool {
        shift.shift_detail_for_date(date).is_some()
    }
}

impl WorkGenerator for BasicWorkGenerator {
    fn generate_work(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        let shifts: Vec<PlannedShift> = Vec::new();

        for date in job.planner_settings().dates(planner_model) {
            for shift in job.shifts_for_standard_set(planner_model.standard_set_id()) {
                if Self::has_shift_on_date(shift, date) {
                }
            }
        }

        WorkResults::with_shifts(job.id(), shifts)
    }
}
