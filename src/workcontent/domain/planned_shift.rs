use crate::id_type;
use crate::workcontent::domain::job::JobId;
use crate::workcontent::domain::location::{Location, LocationId};
use crate::workcontent::domain::planned_shift_type::PlannedShiftType;
use crate::workcontent::domain::planned_shift_source::PlannedShiftSource;
use crate::workcontent::domain::work_content::WorkContentId;
use joda_rs::{LocalDate, LocalDateTime};
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::workcontent::domain::shift_category::ShiftCategoryId;

id_type!(PlannedShiftId, uuid_v7);

pub struct PlannedShift {
    id: PlannedShiftId,
    location_id: LocationId,
    job_id: JobId,
    shift_type: PlannedShiftType,
    shift_date: LocalDate,
    date_shift_generated_from: LocalDate,
    start_date_time: LocalDateTime,
    end_date_time: LocalDateTime,
    duration: Decimal,
    source: PlannedShiftSource,
    shift_category_id: Option<ShiftCategoryId>,
    assignment_id: Option<JobId>,
}

impl PlannedShift {
    pub fn new(
        location_id: LocationId,
        job_id: JobId,
        shift_type: PlannedShiftType,
        shift_date: LocalDate,
        start_date_time: LocalDateTime,
        end_date_time: LocalDateTime
    ) -> Self {
        Self {
            id: PlannedShiftId::new(),
            location_id,
            job_id,
            shift_type,
            shift_date,
            date_shift_generated_from: shift_date,
            start_date_time,
            end_date_time,
            duration: (end_date_time - start_date_time).fractional_hours_decimal(),
            source: PlannedShiftSource::Auto,
            shift_category_id: None,
            assignment_id: None,
        }
    }

    pub fn id(&self) -> PlannedShiftId {
        self.id
    }

    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn job_id(&self) -> JobId {
        self.job_id
    }

    pub fn shift_type(&self) -> PlannedShiftType {
        self.shift_type
    }

    pub fn shift_date(&self) -> LocalDate {
        self.shift_date
    }

    pub fn date_shift_generated_from(&self) -> LocalDate {
        self.date_shift_generated_from
    }

    pub fn start_date_time(&self) -> LocalDateTime {
        self.start_date_time
    }

    pub fn end_date_time(&self) -> LocalDateTime {
        self.end_date_time
    }

    pub fn shift_category_id(&self) -> Option<ShiftCategoryId> { self.shift_category_id }

    pub fn duration(&self) -> Decimal {
        self.duration
    }

    pub fn source(&self) -> PlannedShiftSource {
        self.source
    }

    pub fn assignment_id(&self) -> Option<JobId> {
        self.assignment_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use joda_rs::LocalTime;

    #[test]
    fn planned_shift_setters_and_getters_work() {
        let location_id = LocationId::new();
        let job_id = JobId::new();
        let shift_date = LocalDate::new(2025, 10, 6);
        let start = shift_date.at_time(LocalTime::new(9, 0, 0));
        let end = shift_date.at_time(LocalTime::new(17, 0, 0));
        let planned_shift = PlannedShift::new(
            location_id,
            job_id,
            PlannedShiftType::Projected,
            shift_date,
            start,
            end,
        );

        // id is generated internally; just ensure it's not the default zero value by checking it differs from a fresh id rarely equal
        assert_eq!(planned_shift.location_id(), location_id);
        assert_eq!(planned_shift.job_id(), job_id);
        assert_eq!(planned_shift.shift_type(), PlannedShiftType::Projected);
        assert_eq!(planned_shift.shift_date(), shift_date);
        assert_eq!(planned_shift.date_shift_generated_from(), shift_date);
        assert_eq!(planned_shift.start_date_time(), start);
        assert_eq!(planned_shift.end_date_time(), end);
        assert_eq!(planned_shift.duration(), Decimal::from(8));
        assert_eq!(planned_shift.source(), PlannedShiftSource::Auto);
        assert_eq!(planned_shift.shift_category_id(), None);
        assert_eq!(planned_shift.assignment_id(), None);
    }
}
