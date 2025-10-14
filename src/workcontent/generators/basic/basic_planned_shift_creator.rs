pub struct BasicPlannedShiftCreator;

impl BasicPlannedShiftCreator {
    pub fn new() -> Self { Self }

    pub fn create_planned_shifts_from_work_contents(&self, work_contents: &[String]) -> Vec<String> {
        let mut planned_shifts = Vec::with_capacity(work_contents.len());

        for wc in work_contents {
            planned_shifts.push(Self::create_planned_shift(wc));
        }

        planned_shifts
    }

    fn create_planned_shift(work_content_marker: &str) -> String {
        format!("PLANNED:{}", work_content_marker)
    }
}

#[cfg(test)]
mod tests {
    use super::BasicPlannedShiftCreator;
}
