//! Integration tests -- workflow end-to-end execution
#![allow(clippy::disallowed_methods)]

use spectre_core::workflow::{
    Condition, LoopSpec, StepDef, VariableStore, WorkflowDef, WorkflowExecutor, WorkflowParser,
    WorkflowStatus, WorkflowStore, WorkflowTemplates,
};

#[tokio::test]
async fn test_workflow_yaml_parse_and_execute() {
    let yaml = r#"
name: "integration-test"
description: "Test workflow"
steps:
  - name: "step-1"
    action: "scan"
  - name: "step-2"
    action: "report"
"#;

    let wf = WorkflowParser::from_yaml(yaml).unwrap();
    assert_eq!(wf.name, "integration-test");
    assert_eq!(wf.step_count(), 2);

    let mut executor = WorkflowExecutor::new(wf);
    executor.execute().await.unwrap();
    assert!(matches!(executor.status, WorkflowStatus::Completed));
    assert_eq!(executor.completed_steps(), 2);
}

#[tokio::test]
async fn test_workflow_variable_propagation() {
    let mut wf = WorkflowDef::new("var-propagation-test");
    wf.set_variable("initial", "hello");
    wf.add_step(StepDef::new("step-1", "scan").with_capture("scan_result"));
    wf.add_step(StepDef::new("step-2", "report").with_capture("report_result"));

    let mut executor = WorkflowExecutor::new(wf);
    executor.execute().await.unwrap();

    assert!(executor.variables.get("scan_result").is_some());
    assert!(executor.variables.get("report_result").is_some());
    assert_eq!(executor.variables.get("initial"), Some("hello"));
}

#[tokio::test]
async fn test_workflow_conditional_skip() {
    let mut wf = WorkflowDef::new("conditional-test");
    wf.set_variable("mode", "quick");

    wf.add_step(StepDef::new("always-run", "scan"));
    wf.add_step(
        StepDef::new("full-only", "scan").with_condition(Condition::equals("mode", "full")),
    );
    wf.add_step(
        StepDef::new("quick-only", "scan").with_condition(Condition::equals("mode", "quick")),
    );

    let mut executor = WorkflowExecutor::new(wf);
    executor.execute().await.unwrap();

    assert_eq!(executor.completed_steps(), 2);
    assert_eq!(executor.skipped_steps(), 1);
}

#[tokio::test]
async fn test_workflow_loop_execution() {
    let mut wf = WorkflowDef::new("loop-test");
    wf.set_variable("targets", "10.0.0.1,10.0.0.2,10.0.0.3");
    wf.add_step(
        StepDef::new("scan-each", "scan")
            .with_loop(LoopSpec::new("targets", "current_target"))
            .with_capture("loop_output"),
    );

    let mut executor = WorkflowExecutor::new(wf);
    executor.execute().await.unwrap();

    assert!(matches!(executor.status, WorkflowStatus::Completed));
    let loop_output = executor.variables.get("loop_output");
    assert!(loop_output.is_some());
}

#[tokio::test]
async fn test_workflow_template_execution() {
    let templates = WorkflowTemplates::new();
    let wf = templates.get("recon-to-report").unwrap().clone();
    assert_eq!(wf.step_count(), 3);

    let mut executor = WorkflowExecutor::new(wf);
    executor.execute().await.unwrap();
    assert!(matches!(executor.status, WorkflowStatus::Completed));
    assert_eq!(executor.completed_steps(), 3);
}

#[tokio::test]
async fn test_workflow_persistence_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let store = WorkflowStore::new(dir.path());

    let mut wf = WorkflowDef::new("persist-test");
    wf.description = "Persistence test".to_string();
    wf.add_step(StepDef::new("step-1", "scan"));
    wf.add_step(StepDef::new("step-2", "report"));
    store.save_definition(&wf).unwrap();

    let loaded = store.load_definition("persist-test").unwrap();
    assert_eq!(loaded.name, "persist-test");
    assert_eq!(loaded.step_count(), 2);

    let mut executor = WorkflowExecutor::new(loaded);
    executor.execute().await.unwrap();
    assert!(matches!(executor.status, WorkflowStatus::Completed));

    store.save_state(&executor).unwrap();

    let loaded_state = store.load_state("persist-test").unwrap();
    assert_eq!(loaded_state.completed_steps(), 2);
}

#[test]
fn test_workflow_json_parse() {
    let json = r#"{
        "name": "json-test",
        "description": "JSON workflow",
        "steps": [
            {"name": "step-1", "action": "scan"},
            {"name": "step-2", "action": "report"}
        ]
    }"#;

    let wf = WorkflowParser::from_json(json).unwrap();
    assert_eq!(wf.name, "json-test");
    assert_eq!(wf.step_count(), 2);
}

#[test]
fn test_variable_store_substitution() {
    let mut store = VariableStore::new();
    store.set("target", "10.0.0.1");
    store.set("port_range", "1-1000");

    let result = store.substitute("Scanning ${target} ports ${port_range}");
    assert_eq!(result, "Scanning 10.0.0.1 ports 1-1000");
}

#[tokio::test]
async fn test_workflow_progress_tracking() {
    let mut wf = WorkflowDef::new("progress-test");
    for i in 0..5 {
        wf.add_step(StepDef::new(&format!("step-{}", i), "scan"));
    }

    let mut executor = WorkflowExecutor::new(wf);
    assert!((executor.progress() - 0.0).abs() < f64::EPSILON);

    executor.execute().await.unwrap();
    assert!((executor.progress() - 1.0).abs() < f64::EPSILON);
    assert_eq!(executor.completed_steps(), 5);
}
