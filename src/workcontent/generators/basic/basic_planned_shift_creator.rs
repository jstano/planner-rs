/// Minimal Rust port of the Java PlannedShiftCreator.
///
/// The Java version converts a list of WorkContent domain objects into a list of
/// PlannedShift domain objects by copying fields. The full WorkContent/PlannedShift
/// domains are not yet modeled in Rust in this project, and the Basic generator
/// currently operates with placeholder shift markers (Vec<String> via WorkResults).
///
/// To keep the migration incremental and the code compiling, we provide a
/// BasicPlannedShiftCreator that transforms a slice of work-content markers into
/// a list of planned-shift markers. When real domain types are introduced, this
/// module can be updated to map actual fields one-to-one.
pub struct BasicPlannedShiftCreator;

impl BasicPlannedShiftCreator {
    pub fn new() -> Self { Self }

    /// Mirrors Java's createPlannedShiftsFromWorkContents by iterating over the
    /// inputs and creating corresponding planned shifts. Here we simply wrap
    /// each work content marker string into a planned shift marker string.
    pub fn create_planned_shifts_from_work_contents(&self, work_contents: &[String]) -> Vec<String> {
        let mut planned_shifts = Vec::with_capacity(work_contents.len());

        for wc in work_contents {
            planned_shifts.push(Self::create_planned_shift(wc));
        }

        planned_shifts
    }

    /// Placeholder conversion of a single WorkContent marker into a PlannedShift marker.
    /// In the future, this would construct a PlannedShift struct and copy fields
    /// such as job, shift type, dates, start/end times, duration, source, etc.
    fn create_planned_shift(work_content_marker: &str) -> String {
        format!("PLANNED:{}", work_content_marker)
    }
}

#[cfg(test)]
mod tests {
    use super::BasicPlannedShiftCreator;

    #[test]
    fn creates_planned_shifts_from_markers() {
        let creator = BasicPlannedShiftCreator::new();
        let inputs = vec![
            "BASIC:2025-01-01:SHIFT:1".to_string(),
            "BASIC:2025-01-02:SHIFT:2".to_string(),
        ];

        let planned = creator.create_planned_shifts_from_work_contents(&inputs);
        assert_eq!(planned.len(), 2);
        assert_eq!(planned[0], "PLANNED:BASIC:2025-01-01:SHIFT:1");
        assert_eq!(planned[1], "PLANNED:BASIC:2025-01-02:SHIFT:2");
    }
}
