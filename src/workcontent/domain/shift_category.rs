use uuid::Uuid;
use crate::id_type;
use crate::workcontent::domain::job::JobId;
use crate::workcontent::domain::location::LocationId;

id_type!(ShiftCategoryId, uuid_v4);

pub struct ShiftCategory {
    id: ShiftCategoryId,
    location_id: LocationId,
    name: String,
    code: String,
    training_shift: bool,
    contract_shift: bool,
}

impl ShiftCategory {
    pub fn new(location_id: LocationId,
               name: String,
               code: String,
               training_shift: bool,
               contract_shift: bool) -> Self {
        Self {
            id: ShiftCategoryId::new(),
            location_id,
            name,
            code,
            training_shift,
            contract_shift,
        }
    }

    pub fn id(&self) -> ShiftCategoryId {
        self.id
    }

    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn training_shift(&self) -> bool {
        self.training_shift
    }

    pub fn contract_shift(&self) -> bool {
        self.contract_shift
    }
}
