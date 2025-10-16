#[derive(Debug, Default, Clone)]
pub struct WorkContentLogDetail {
    work_in_minutes: f64,
    formula: String,
}

impl WorkContentLogDetail {
    pub fn new(work_in_minutes: f64, formula: String) -> Self {
        Self {
            work_in_minutes,
            formula,
        }
    }
    pub fn work_in_minutes(&self) -> f64 {
        self.work_in_minutes
    }
    pub fn formula(&self) -> &str {
        &self.formula
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_fields_and_getters_work() {
        let detail = WorkContentLogDetail::new(12.5, "a*b + c".to_string());
        assert_eq!(detail.work_in_minutes(), 12.5);
        assert_eq!(detail.formula(), "a*b + c");
    }

    #[test]
    fn default_is_zero_and_empty_formula() {
        let detail = WorkContentLogDetail::default();
        assert_eq!(detail.work_in_minutes(), 0.0);
        assert_eq!(detail.formula(), "");
    }

    #[test]
    fn clone_and_debug_behavior() {
        let detail = WorkContentLogDetail::new(7.75, "x / y".to_string());
        let cloned = detail.clone();
        assert_eq!(cloned.work_in_minutes(), 7.75);
        assert_eq!(cloned.formula(), "x / y");

        // Ensure Debug prints something useful (non-empty and contains type name)
        let dbg = format!("{:?}", detail);
        assert!(dbg.contains("WorkContentLogDetail"));
        assert!(!dbg.is_empty());
    }
}
