use joda_rs::{LocalDate, LocalDateTime};
use uuid::Uuid;
use crate::id_type;
use crate::workcontent::domain::job::JobId;
use crate::workcontent::domain::plan_type::PlanType;
use crate::workcontent::domain::location::Location;
use crate::workcontent::domain::shift_source::ShiftSource;
use crate::workcontent::domain::work_content::WorkContentId;

id_type!(PlannedShiftId, uuid_v4);

#[derive(Default)]
pub struct PlannedShift {
    id: i32,
    job_id: Option<JobId>,
    property: Option<Location>,
    shift_type: Option<PlanType>,
    shift_date: Option<LocalDate>,
    date_shift_generated_from: Option<LocalDate>,
    start_date_time: Option<LocalDateTime>,
    end_date_time: Option<LocalDateTime>,
    // shift_category: Option<ShiftCategory>,
    duration: f64,
    source: Option<ShiftSource>,
    assignment_id: Option<JobId>,
}

impl PlannedShift {
    pub fn new() -> Self { Self::default() }

    // id
    pub fn id(&self) -> i32 { self.id }
    pub fn set_id(&mut self, id: i32) { self.id = id; }

    // job (Assignment in Java)
    pub fn job_id(&self) -> Option<JobId> { self.job_id }
    pub fn set_job_id(&mut self, job_id: Option<JobId>) { self.job_id = job_id; }

    // property
    pub fn property(&self) -> Option<&Location> { self.property.as_ref() }
    pub fn set_property(&mut self, property: Option<Location>) { self.property = property; }

    // shift type
    pub fn shift_type(&self) -> Option<PlanType> { self.shift_type }
    pub fn set_shift_type(&mut self, shift_type: Option<PlanType>) { self.shift_type = shift_type; }

    // shift date
    pub fn shift_date(&self) -> Option<LocalDate> { self.shift_date }
    pub fn set_shift_date(&mut self, shift_date: Option<LocalDate>) { self.shift_date = shift_date; }

    // date shift generated from
    pub fn date_shift_generated_from(&self) -> Option<LocalDate> { self.date_shift_generated_from }
    pub fn set_date_shift_generated_from(&mut self, date: Option<LocalDate>) { self.date_shift_generated_from = date; }

    // start date time
    pub fn start_date_time(&self) -> Option<LocalDateTime> { self.start_date_time }
    pub fn set_start_date_time(&mut self, value: Option<LocalDateTime>) { self.start_date_time = value; }

    // end date time
    pub fn end_date_time(&self) -> Option<LocalDateTime> { self.end_date_time }
    pub fn set_end_date_time(&mut self, value: Option<LocalDateTime>) { self.end_date_time = value; }

    // shift category
    // pub fn shift_category(&self) -> Option<ShiftCategory> { self.shift_category }
    // pub fn set_shift_category(&mut self, category: Option<ShiftCategory>) { self.shift_category = category; }

    // duration
    pub fn duration(&self) -> f64 { self.duration }
    pub fn set_duration(&mut self, duration: f64) { self.duration = duration; }

    // source
    pub fn source(&self) -> Option<ShiftSource> { self.source }
    pub fn set_source(&mut self, source: Option<ShiftSource>) { self.source = source; }

    // assignment (Assignment in Java)
    pub fn assignment_id(&self) -> Option<JobId> { self.assignment_id }
    pub fn set_assignment_id(&mut self, assignment_id: Option<JobId>) { self.assignment_id = assignment_id; }
}

#[cfg(test)]
mod tests {
    use joda_rs::LocalTime;
    use super::*;

    #[test]
    fn planned_shift_setters_and_getters_work() {
        let mut ps = PlannedShift::new();
        ps.set_id(42);
        ps.set_shift_type(Some(PlanType::Forecast));
        let date = LocalDate::new(2025, 1, 2);
        ps.set_shift_date(Some(date));
        ps.set_date_shift_generated_from(Some(date));
        let start = LocalDateTime::of_date_time(date, LocalTime::new(9, 0, 0));
        let end = LocalDateTime::of_date_time(date, LocalTime::new(17, 0, 0));
        ps.set_start_date_time(Some(start));
        ps.set_end_date_time(Some(end));
        // ps.set_shift_category(Some(ShiftCategory::Unknown));
        ps.set_duration(8.0);
        ps.set_source(Some(ShiftSource::Auto));

        assert_eq!(ps.id(), 42);
        assert_eq!(ps.shift_type(), Some(PlanType::Forecast));
        assert_eq!(ps.shift_date(), Some(date));
        assert_eq!(ps.date_shift_generated_from(), Some(date));
        assert_eq!(ps.start_date_time(), Some(start));
        assert_eq!(ps.end_date_time(), Some(end));
        // assert_eq!(ps.shift_category(), Some(ShiftCategory::Unknown));
        assert_eq!(ps.duration(), 8.0);
        assert_eq!(ps.source(), Some(ShiftSource::Auto));
    }
}
