use joda_rs::LocalDate;
use crate::workcontent::domain::job::{Job};
use crate::workcontent::domain::job_shift::JobShift;
use crate::workcontent::domain::planner_model::PlannerModel;
use crate::workcontent::generators::work_generators::WorkResults;

pub trait BasicStandardsProcessor {
    fn process(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults;
}

pub struct BasicStandardsProcessorImpl;

impl BasicStandardsProcessorImpl {
    pub fn new() -> Self { Self }

    fn has_shift_on_date(&self, shift: &JobShift, date: LocalDate) -> bool {
        shift.shift_detail_for_date(date).is_some()
    }
}

impl BasicStandardsProcessor for BasicStandardsProcessorImpl {
    fn process(&self, planner_model: &PlannerModel, job: &Job) -> WorkResults {
        // Minimal port: iterate effective dates and the job's shifts for the active standard set.
        // For each date/shift that has a detail, add a simple string describing the planned work.
        let mut shifts: Vec<String> = Vec::new();

        for date in job.planner_settings().dates(planner_model) {
            for shift in job.shifts_for_standard_set(planner_model.standard_set_id()) {
                if self.has_shift_on_date(shift, date) {
                    // We don't yet have WorkContent/PlannedShift types wired up in Rust, so
                    // return a simple marker string capturing intent, keeping changes minimal.
                    shifts.push(format!("BASIC:{}:SHIFT:{:?}", date, shift.id()));
                }
            }
        }

        WorkResults::with_shifts(job.id(), shifts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Just ensure the trait is object-safe and impl is constructible
    fn require_impl_trait<T: BasicStandardsProcessor>() {}

    #[test]
    fn trait_is_object_safe() {
        fn _accepts_box(_: Box<dyn BasicStandardsProcessor>) {}
        fn _accepts_ref(_: &dyn BasicStandardsProcessor) {}
        let _ = _accepts_box as fn(Box<dyn BasicStandardsProcessor>);
        let _ = _accepts_ref as fn(&dyn BasicStandardsProcessor);
    }

    #[test]
    fn impl_can_be_constructed() {
        let _impl = BasicStandardsProcessorImpl::new();
        require_impl_trait::<BasicStandardsProcessorImpl>();
    }
}
