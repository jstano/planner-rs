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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn business_driver_new_and_getters_work() {
        let id = BusinessDriverId::new();
        let location_id = LocationId::new();

        let bd = BusinessDriver::new(id, location_id);

        assert_eq!(bd.id(), id);
        assert_eq!(bd.location_id(), location_id);
    }
}
