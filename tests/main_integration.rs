use planner::workcontent::domain::job::Job;
use planner::workcontent::domain::location::LocationId;
use planner::workcontent::domain::planner_model::PlannerModel;
use planner::workcontent::domain::planner_settings::PlannerSettings;
use planner::workcontent::domain::standard_type::StandardType;
use planner::workcontent::main::main::generate_work_content;
use date_range_rs::DateRange;
use joda_rs::LocalDate;
use std::collections::HashMap;

#[test]
fn integration_single_salaried_job_generates_correct_labor_data() {
    // Arrange: planner dates covering several days
    let start = LocalDate::new(2025, 1, 1);
    let end = LocalDate::new(2025, 1, 5); // inclusive range -> 5 days
    let dates = DateRange::new(start, end);

    let location_id = LocationId::new();
    let standard_set_id = planner::workcontent::domain::standard_set::StandardSetId::new();

    // PlannerSettings configured for SALARIED with effective dates that include planner dates
    let mut settings = PlannerSettings::default();
    // Force salaried generator path
    settings.standard_type = StandardType::SALARIED;
    // Ensure effective dates include the whole year to match planner dates
    settings.effective_dates = DateRange::new(
        LocalDate::new(2025, 1, 1),
        LocalDate::new(2025, 12, 31),
    );

    // Create a salaried job with no shifts/standards (expected hours will be 0.0 per day)
    let job = Job::new(location_id, settings, vec![], vec![]);

    let planner_model = PlannerModel::new(
        dates,
        LocationId::new(),
        standard_set_id,
        vec![job],
        vec![],
        HashMap::new(),
    );

    // Act
    let results = generate_work_content(planner_model);

    // Assert: exactly one result corresponding to the job
    assert_eq!(results.len(), 1);

    let result = &results[0];
    // Salaried generator should produce labor_data and no shifts
    assert!(result.shifts().is_none(), "Salaried should not produce shifts");
    let labor = result
        .labor_data()
        .expect("Expected labor data for salaried job");

    // Should generate one labor entry per planner date
    assert_eq!(labor.len(), 5, "Expected a labor entry for each planner date");

    // All entries should be within the planner date range and have 0.0 hours (no shifts/standards)
    for (i, ld) in labor.iter().enumerate() {
        assert!(ld.date() >= start && ld.date() <= end, "labor date out of range: {:?}", ld.date());
        assert_eq!(ld.hours(), 0.0, "expected 0.0 hours on index {} (date {:?})", i, ld.date());
    }
}
