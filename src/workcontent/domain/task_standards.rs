use crate::workcontent::domain::environment::Environment;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskStandardFrequency {
    environment: Environment,
    frequency: i32,
}

impl TaskStandardFrequency {
    pub fn new(environment: Environment, frequency: i32) -> Self {
        Self {
            environment,
            frequency,
        }
    }
    pub fn environment(&self) -> &Environment {
        &self.environment
    }
    pub fn frequency(&self) -> i32 {
        self.frequency
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskStandardRange {
    min_inclusive: i32,
    max_inclusive: i32,
}

impl TaskStandardRange {
    pub fn new(min_inclusive: i32, max_inclusive: i32) -> Self {
        Self {
            min_inclusive,
            max_inclusive,
        }
    }
    pub fn contains_value(&self, value: i32) -> bool {
        value >= self.min_inclusive && value <= self.max_inclusive
    }
}

#[derive(Debug, Clone)]
pub struct TaskStandardDetail {
    reasonable_expectancy: f64,
    number_of_items: i32,
    ranges: Vec<(TaskStandardRange, Vec<TaskStandardFrequency>)>,
}

impl TaskStandardDetail {
    pub fn new(reasonable_expectancy: f64, number_of_items: i32) -> Self {
        Self {
            reasonable_expectancy,
            number_of_items,
            ranges: Vec::new(),
        }
    }
    pub fn with_ranges(
        mut self,
        ranges: Vec<(TaskStandardRange, Vec<TaskStandardFrequency>)>,
    ) -> Self {
        self.ranges = ranges;
        self
    }
    pub fn reasonable_expectancy(&self) -> f64 {
        self.reasonable_expectancy
    }
    pub fn number_of_items(&self) -> i32 {
        self.number_of_items
    }
    pub fn task_standard_ranges(&self) -> &[(TaskStandardRange, Vec<TaskStandardFrequency>)] {
        &self.ranges
    }
}
