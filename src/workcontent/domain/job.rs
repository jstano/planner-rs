use uuid::Uuid;
use crate::id_type;
use crate::workcontent::domain::job_shift::JobShift;
use crate::workcontent::domain::planner_settings::PlannerSettings;
use crate::workcontent::domain::location::LocationId;
use crate::workcontent::domain::salaried_standard::SalariedStandard;
use crate::workcontent::domain::standard_set::{StandardSet, StandardSetId};

id_type!(JobId, uuid_v4);

pub struct Job {
    id: JobId,
    property_id: LocationId,
    planner_settings: PlannerSettings,
    shifts: Vec<JobShift>,
    salaried_standards: Vec<SalariedStandard>,
}

impl Job {
    pub fn new(property_id: LocationId,
               planner_settings: PlannerSettings,
               shifts: Vec<JobShift>,
               salaried_standards: Vec<SalariedStandard>) -> Self {
        Self {
            id: JobId::new(),
            property_id,
            planner_settings,
            shifts,
            salaried_standards
        }
    }

    pub fn id(&self) -> JobId {
        self.id
    }

    pub fn planner_settings(&self) -> PlannerSettings {
        PlannerSettings::default()
    }

    pub fn shifts(&self) -> &[JobShift] {
        &self.shifts
    }

    pub fn shifts_for_standard_set(&self, standard_set_id: StandardSetId) -> Vec<&JobShift> {
        self.shifts.iter()
            .filter(|shift| *shift.standard_set_id() == standard_set_id)
            .collect()
    }

    pub fn salaried_standards(&self) -> &[SalariedStandard] {
        &self.salaried_standards
    }

    pub fn salaried_standard_for_standard_set_and_shift(&self, standard_set_id: StandardSetId, shift: &JobShift) -> Option<&SalariedStandard> {
        self.salaried_standards.iter()
            .find(|standard|
                standard.standard_set_id == standard_set_id &&
                    standard.shift.id() == shift.id())
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            id: JobId::new(),
            property_id: LocationId::new(),
            planner_settings: PlannerSettings::default(),
            shifts: Vec::new(),
            salaried_standards: Vec::new(),
        }
    }
}
