use joda_rs::{DayOfWeek, LocalDate};
use uuid::Uuid;
use crate::id_type;
use crate::workcontent::domain::job::{Job, JobId};
use crate::workcontent::domain::planner_settings::PlannerSettings;
use crate::workcontent::domain::location::LocationId;
use crate::workcontent::domain::standard_set::{StandardSet, StandardSetId};

id_type!(JobShiftId, uuid_v4);

pub struct JobShift {
    id: JobShiftId,
    job_id: JobId,
    standard_set_id: StandardSetId,
    name: String,
    sequence: u32,
    shift_definitions: Vec<JobShiftDefinition>,
}

impl JobShift {
    pub fn new(job_id: JobId,
               standard_set_id: StandardSetId,
               name: String,
               sequence: u32,
               shift_definitions: Vec<JobShiftDefinition>) -> Self {
        Self {
            id: JobShiftId::new(),
            job_id,
            standard_set_id,
            name,
            sequence,
            shift_definitions
        }
    }

    pub fn id(&self) -> &JobShiftId {
        &self.id
    }

    pub fn standard_set_id(&self) -> &StandardSetId {
        &self.standard_set_id
    }

    pub fn shift_detail_for_date(&self, date: LocalDate) -> Option<&JobShiftDefinition> {
        self.shift_definitions.iter().find(|shift_definition| shift_definition.day_of_week == date.day_of_week())
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            id: JobShiftId::new(),
            job_id: JobId::new(),
            standard_set_id: StandardSetId::new(),
            name: String::new(),
            sequence: 0,
            shift_definitions: Vec::new(),
        }
    }
}

pub struct JobShiftDefinition {
    job_shift: JobShift,
    day_of_week: DayOfWeek,
    start_time: LocalDate,
    end_time: LocalDate,
    hours_before: f64,
    hours_after: f64,
    min_number_shifts: u32,
}

impl JobShiftDefinition {
}
