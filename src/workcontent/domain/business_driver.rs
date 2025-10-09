use crate::id_type;
use crate::workcontent::domain::job::JobId;
use crate::workcontent::domain::job_shift::JobShift;
use crate::workcontent::domain::location::LocationId;
use crate::workcontent::domain::planner_settings::PlannerSettings;
use crate::workcontent::domain::salaried_standard::SalariedStandard;

id_type!(BusinessDriverId, uuid_v4);

pub struct BusinessDriver {
    id: BusinessDriverId,
    location_id: LocationId,
}

impl BusinessDriver {
    pub fn new(id: BusinessDriverId, location_id: LocationId) -> Self {
        Self {
            id,
            location_id
        }
    }

    pub fn id(&self) -> BusinessDriverId {
        self.id
    }

    pub fn location_id(&self) -> LocationId {
        self.location_id
    }
}
