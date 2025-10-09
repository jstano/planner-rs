use crate::workcontent::domain::business_driver::BusinessDriverId;
use crate::workcontent::domain::location::LocationId;
use std::collections::HashMap;
use joda_rs::LocalDate;

pub struct BusinessDriverValues {
    business_driver_id: BusinessDriverId,
    values: HashMap<LocalDate, u32>,
}
impl BusinessDriverValues {}
