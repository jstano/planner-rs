use crate::workcontent::domain::job::{Job, JobId};
use crate::workcontent::domain::job_shift::JobShift;
use crate::workcontent::domain::salary_mode::SalaryMode;
use crate::workcontent::domain::standard_set::{StandardSet, StandardSetId};

pub struct SalariedStandard {
    pub job_id: JobId,
    pub standard_set_id: StandardSetId,
    pub shift: JobShift,
    pub salary_mode: SalaryMode,
    pub hours_per_week: f64,
    pub vacation_hours_per_year: f64,
    pub hours_per_year: f64,
}

impl SalariedStandard {
}
