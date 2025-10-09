use joda_rs::{LocalDate, LocalDateTime};
use uuid::Uuid;
use crate::id_type;
use crate::workcontent::domain::job::JobId;
use crate::workcontent::domain::plan_type::PlanType;
use crate::workcontent::domain::location::{Location, LocationId};

id_type!(WorkContentId, uuid_v4);

#[derive(Debug, Clone)]
pub struct WorkContent {
    id: WorkContentId,
    job_id: JobId,
    property_id: LocationId,
    shift_type: PlanType,
    shift_date: LocalDate,
    earliest_start_date_time: LocalDateTime,
    preferred_start_date_time: LocalDateTime,
    latest_end_date_time: LocalDateTime,
    calculated_start_date_time: LocalDateTime,
    calculated_end_date_time: LocalDateTime,
    calculated_hours: f64,
    adjusted_hours: f64,
    locked: bool,
    description: String,
    min_number_employees: u32,
    min_skill_level: u32,
    distributed_to_date_time: LocalDateTime,
}

impl WorkContent {
    pub fn new(
        job_id: JobId,
        property_id: LocationId,
        shift_type: PlanType,
        shift_date: LocalDate,
        earliest_start_date_time: LocalDateTime,
        preferred_start_date_time: LocalDateTime,
        latest_end_date_time: LocalDateTime,
        calculated_start_date_time: LocalDateTime,
        calculated_end_date_time: LocalDateTime,
        calculated_hours: f64,
        adjusted_hours: f64,
        locked: bool,
        description: String,
        min_number_employees: u32,
        min_skill_level: u32,
        distributed_to_date_time: LocalDateTime
    ) -> Self {
        Self {
            id: WorkContentId::new(),
            job_id,
            property_id,
            shift_type,
            shift_date,
            earliest_start_date_time,
            preferred_start_date_time,
            latest_end_date_time,
            calculated_start_date_time,
            calculated_end_date_time,
            calculated_hours,
            adjusted_hours,
            locked,
            description,
            min_number_employees,
            min_skill_level,
            distributed_to_date_time,
        }
    }

    pub fn id(&self) -> WorkContentId { self.id }
    pub fn job_id(&self) -> JobId { self.job_id }
    pub fn property_id(&self) -> LocationId { self.property_id }
    pub fn shift_type(&self) -> PlanType { self.shift_type }
    pub fn shift_date(&self) -> LocalDate { self.shift_date }
    pub fn earliest_start_date_time(&self) -> LocalDateTime { self.earliest_start_date_time }
    pub fn preferred_start_date_time(&self) -> LocalDateTime { self.preferred_start_date_time }
    pub fn latest_end_date_time(&self) -> LocalDateTime { self.latest_end_date_time }
    pub fn calculated_start_date_time(&self) -> LocalDateTime { self.calculated_start_date_time }
    pub fn calculated_end_date_time(&self) -> LocalDateTime { self.calculated_end_date_time }
    pub fn calculated_hours(&self) -> f64 { self.calculated_hours }
    pub fn adjusted_hours(&self) -> f64 { self.adjusted_hours }
    pub fn is_locked(&self) -> bool { self.locked }
    pub fn description(&self) -> &str { self.description.as_str() }
    pub fn min_number_employees(&self) -> u32 { self.min_number_employees }
    pub fn min_skill_level(&self) -> u32 { self.min_skill_level }
    pub fn distributed_to_date_time(&self) -> LocalDateTime { self.distributed_to_date_time }
}
