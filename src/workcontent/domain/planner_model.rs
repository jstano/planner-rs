use std::collections::HashMap;
use date_range_rs::DateRange;
use crate::workcontent::domain::business_driver::{BusinessDriver, BusinessDriverId};
use crate::workcontent::domain::job::Job;
use crate::workcontent::domain::standard_set::{StandardSet, StandardSetId};
use crate::workcontent::domain::location::{Location, LocationId};

pub struct PlannerModel {
    dates: DateRange,
    location_id: LocationId,
    standard_set_id: StandardSetId,
    jobs: Vec<Job>,
    business_drivers: Vec<BusinessDriver>,
    business_driver_values: HashMap<BusinessDriverId, u32>,
}

impl PlannerModel {
    pub fn new(dates: DateRange,
               location_id: LocationId,
               standard_set_id: StandardSetId,
               jobs: Vec<Job>,
               business_drivers: Vec<BusinessDriver>,
               business_driver_values: HashMap<BusinessDriverId, u32>) -> Self {
        Self {
            dates,
            location_id,
            standard_set_id,
            jobs,
            business_drivers,
            business_driver_values,
        }
    }

    pub fn dates(&self) -> DateRange {
        self.dates
    }

    pub fn location_id(&self) -> &LocationId {
        &self.location_id
    }

    pub fn standard_set_id(&self) -> StandardSetId {
        self.standard_set_id
    }

    pub fn jobs(&self) -> &[Job] {
        &self.jobs
    }

    pub fn jobs_iter(&self) -> impl Iterator<Item = &Job> {
        self.jobs.iter()
    }

    pub fn business_drivers(&self) -> &[BusinessDriver] {
        &self.business_drivers
    }

    pub fn business_driver_values(&self) -> &HashMap<BusinessDriverId, u32> {
        &self.business_driver_values
    }
}
