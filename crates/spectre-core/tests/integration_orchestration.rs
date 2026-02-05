//! Integration tests -- orchestration module
#![allow(clippy::disallowed_methods)]

use spectre_core::orchestration::{
    AdaptiveTiming, Checkpoint, CheckpointStore, ScanChainBuilder, ScanProfile, ScanProfileStore,
    ScanScheduler, ScanStep, ScheduleEntry, ScheduleExpression, StepCondition, TemplateLibrary,
};
use spectre_core::scan::{ScanResult, ScanType, TimingTemplate};

#[test]
fn test_scan_chain_builder_integration() {
    let chain = ScanChainBuilder::new("full-recon")
        .step(
            ScanStep::new("discovery", ScanType::Connect)
                .with_condition(StepCondition::Always)
                .with_description("Initial host discovery"),
        )
        .step(
            ScanStep::new("port-scan", ScanType::Syn)
                .with_condition(StepCondition::HasOpenPorts)
                .with_description("SYN port scan"),
        )
        .step(
            ScanStep::new("service-detect", ScanType::Connect)
                .with_condition(StepCondition::HasOpenPorts)
                .with_service_detection()
                .with_description("Service version detection"),
        )
        .description("Complete reconnaissance chain")
        .build();

    assert_eq!(chain.name, "full-recon");
    assert_eq!(chain.steps.len(), 3);
    assert_eq!(chain.description, "Complete reconnaissance chain");
}

#[test]
fn test_scan_template_library() {
    let library = TemplateLibrary::new();

    assert!(library.get("quick-recon").is_some());
    assert!(library.get("full-audit").is_some());
    assert!(library.get("stealth-enum").is_some());
    assert!(library.get("web-enum").is_some());
    assert!(library.get("udp-scan").is_some());
    assert!(library.count() >= 5);

    let recon = library.search_by_tag("recon");
    assert!(!recon.is_empty());

    let all = library.list();
    assert!(all.len() >= 5);
    // Verify sorted by name
    for window in all.windows(2) {
        assert!(window[0].name <= window[1].name);
    }
}

#[test]
fn test_scan_scheduling_integration() {
    let mut scheduler = ScanScheduler::new();

    scheduler.add(
        ScheduleEntry::new("hourly-recon", ScheduleExpression::hourly(), "quick-recon")
            .with_targets(vec!["10.0.0.0/24".to_string()]),
    );
    scheduler.add(
        ScheduleEntry::new("nightly-audit", ScheduleExpression::daily(2), "full-audit")
            .with_targets(vec!["10.0.0.0/24".to_string(), "172.16.0.0/16".to_string()]),
    );
    scheduler.add(ScheduleEntry::new(
        "weekly-web",
        ScheduleExpression::weekly(1, 9),
        "web-enum",
    ));

    assert_eq!(scheduler.count(), 3);

    let midnight = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2025, 6, 16, 0, 0, 0).unwrap();
    let due = scheduler.due_entries(&midnight);
    assert!(due.iter().any(|e| e.name == "hourly-recon"));

    assert!(scheduler.set_enabled("weekly-web", false));
    assert!(scheduler.remove("nightly-audit"));
    assert_eq!(scheduler.count(), 2);
}

#[test]
fn test_scan_profile_store() {
    let mut store = ScanProfileStore::new();

    let profile = ScanProfile::new("stealth", ScanType::Syn)
        .with_rate_limit(100)
        .with_description("Stealth scan profile");

    store.add(profile);

    let loaded = store.get("stealth");
    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.name, "stealth");
    assert_eq!(loaded.rate_limit, Some(100));
}

#[test]
fn test_checkpoint_resume() {
    let dir = tempfile::tempdir().unwrap();
    let store = CheckpointStore::new(dir.path());

    let mut checkpoint = Checkpoint::new("cp-1", "long-scan");
    checkpoint.remaining_targets = vec![
        "10.0.0.1".to_string(),
        "10.0.0.2".to_string(),
        "10.0.0.3".to_string(),
    ];
    checkpoint.mark_target_complete("10.0.0.1");

    store.save(&checkpoint).unwrap();

    let loaded = store.load("cp-1").unwrap();
    assert_eq!(loaded.scan_name, "long-scan");
    assert_eq!(loaded.completed_targets.len(), 1);
    assert_eq!(loaded.remaining_targets.len(), 2);
    assert!(loaded.completed_targets.contains(&"10.0.0.1".to_string()));
}

#[test]
fn test_adaptive_timing_adjustment() {
    let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);

    // Simulate good responses (no loss)
    for _ in 0..20 {
        timing.record_response(std::time::Duration::from_millis(50));
    }

    let fast_delay = timing.current_delay();

    // Simulate packet loss (timeouts)
    for _ in 0..10 {
        timing.record_timeout();
    }

    let slow_delay = timing.current_delay();

    assert!(
        slow_delay >= fast_delay,
        "Delay should increase after packet loss: fast={:?}, slow={:?}",
        fast_delay,
        slow_delay
    );
}

#[test]
fn test_step_condition_evaluation_always() {
    assert!(StepCondition::Always.evaluate(&[]));
}

#[test]
fn test_step_condition_has_results() {
    assert!(!StepCondition::HasResults.evaluate(&[]));

    let result = ScanResult {
        host: "10.0.0.1".to_string(),
        ip: spectre_core::scan::Target::Ip("10.0.0.1".parse().unwrap()),
        hostname: None,
        ports: vec![],
        os: None,
        scan_time: std::time::Duration::from_millis(50),
    };
    assert!(StepCondition::HasResults.evaluate(&[result]));
}

#[test]
fn test_schedule_expression_parsing() {
    let sched = ScheduleExpression::parse("*/15 * * * *").unwrap();
    let t = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2025, 6, 15, 14, 0, 0).unwrap();
    assert!(sched.matches(&t));

    let t2 = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2025, 6, 15, 14, 7, 0).unwrap();
    assert!(!sched.matches(&t2));

    assert!(ScheduleExpression::parse("invalid").is_err());
    assert!(ScheduleExpression::parse("* * *").is_err());
}
