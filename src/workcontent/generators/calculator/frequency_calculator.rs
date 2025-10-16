use crate::workcontent::common::numbers::round_raw_hours;
use crate::workcontent::domain::environment::Environment;
use crate::workcontent::domain::task_standards::{TaskStandardDetail, TaskStandardFrequency, TaskStandardRange};
use crate::workcontent::generators::calculator::work_content_log::WorkContentLogDetail;

struct FrequencyCalculator;

impl FrequencyCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_frequency_minutes(
        &self,
        task_standard_detail: &TaskStandardDetail,
        business_driver_value: i32,
        environment: Environment,
    ) -> Option<WorkContentLogDetail> {
        let reasonable_expectancy_for_items = task_standard_detail.reasonable_expectancy()
            * task_standard_detail.number_of_items() as f64;

        for (range, freqs) in task_standard_detail.task_standard_ranges() {
            if range.contains_value(business_driver_value) {
                for freq in freqs {
                    if freq.environment().id() == environment.id() {
                        let total_minutes =
                            reasonable_expectancy_for_items * freq.frequency() as f64;
                        let rounded = round_raw_hours(total_minutes);

                        return Some(WorkContentLogDetail::new(
                            rounded,
                            format!(
                                "{:.2} * {}",
                                reasonable_expectancy_for_items,
                                freq.frequency()
                            )
                        ));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_detail(re: f64, items: i32, ranges: Vec<(TaskStandardRange, Vec<TaskStandardFrequency>)>) -> TaskStandardDetail {
        TaskStandardDetail::new(re, items).with_ranges(ranges)
    }

    #[test]
    fn calculates_minutes_when_range_and_environment_match() {
        let env1 = Environment::new(1);
        let env2 = Environment::new(2);
        let detail = make_detail(
            1.2345,
            3,
            vec![
                (
                    TaskStandardRange::new(0, 100),
                    vec![
                        TaskStandardFrequency::new(env2, 5),
                        TaskStandardFrequency::new(env1, 2),
                    ],
                ),
            ],
        );

        let calc = FrequencyCalculator::new();
        let res = calc.calculate_frequency_minutes(&detail, 50, env1).expect("expected some result");

        // reasonable_expectancy_for_items = 1.2345 * 3 = 3.7035
        // total = 3.7035 * 2 = 7.407 -> rounded to 7.41 (2 decimals)
        assert!((res.work_in_minutes() - 7.41).abs() < 1e-12);
        assert_eq!(res.formula(), "3.70 * 2");
    }

    #[test]
    fn returns_none_when_no_range_matches_business_driver() {
        let env = Environment::new(1);
        let detail = make_detail(
            2.0,
            1,
            vec![
                (TaskStandardRange::new(10, 20), vec![TaskStandardFrequency::new(env, 3)])
            ],
        );
        let calc = FrequencyCalculator::new();
        let res = calc.calculate_frequency_minutes(&detail, 5, env);
        assert!(res.is_none());
    }

    #[test]
    fn returns_none_when_environment_not_found_in_matching_range() {
        let env1 = Environment::new(1);
        let env2 = Environment::new(2);
        let detail = make_detail(
            1.5,
            4,
            vec![
                (TaskStandardRange::new(0, 10), vec![TaskStandardFrequency::new(env1, 3)])
            ],
        );
        let calc = FrequencyCalculator::new();
        let res = calc.calculate_frequency_minutes(&detail, 7, env2);
        assert!(res.is_none());
    }

    #[test]
    fn picks_correct_range_and_env_among_multiple() {
        let env1 = Environment::new(1);
        let env2 = Environment::new(2);
        let env3 = Environment::new(3);
        let detail = make_detail(
            0.75, // RE per item
            8,    // items -> 6.0 total per occurrence
            vec![
                (
                    TaskStandardRange::new(0, 5),
                    vec![TaskStandardFrequency::new(env1, 10)],
                ),
                (
                    TaskStandardRange::new(6, 10),
                    vec![
                        TaskStandardFrequency::new(env2, 4),
                        TaskStandardFrequency::new(env3, 7),
                    ],
                ),
            ],
        );

        let calc = FrequencyCalculator::new();

        // In range 6..=10, for env3 frequency is 7
        let res = calc.calculate_frequency_minutes(&detail, 9, env3).expect("some");
        // RE_for_items = 0.75 * 8 = 6.0; total = 6 * 7 = 42.0
        assert!((res.work_in_minutes() - 42.0).abs() < 1e-12);
        assert_eq!(res.formula(), "6.00 * 7");

        // Same range but env2 -> freq 4
        let res2 = calc.calculate_frequency_minutes(&detail, 9, env2).expect("some");
        assert!((res2.work_in_minutes() - 24.0).abs() < 1e-12);
        assert_eq!(res2.formula(), "6.00 * 4");
    }
}
