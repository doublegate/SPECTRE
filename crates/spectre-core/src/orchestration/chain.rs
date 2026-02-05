//! Scan chaining — chain multiple scans, output feeds next scan
//!
//! Supports conditional execution based on prior scan results.

#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, warn};

use crate::scan::{PortState, ScanResult, ScanType};

/// Condition that determines whether a scan step should execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepCondition {
    /// Always execute
    Always,
    /// Execute only if a specific port is open on any target
    PortOpen(u16),
    /// Execute only if a specific service is detected
    ServiceDetected(String),
    /// Execute only if any host has open ports
    HasOpenPorts,
    /// Execute only if the previous step produced results
    HasResults,
    /// Execute only if a specific OS is detected
    OsDetected(String),
    /// Custom condition with a label (evaluated externally)
    Custom(String),
}

impl StepCondition {
    /// Evaluate this condition against scan results
    pub fn evaluate(&self, results: &[ScanResult]) -> bool {
        match self {
            Self::Always | Self::Custom(_) => true, // Custom conditions default to true
            Self::PortOpen(port) => results.iter().any(|r| {
                r.ports
                    .iter()
                    .any(|p| p.port == *port && p.state == PortState::Open)
            }),
            Self::ServiceDetected(service) => results.iter().any(|r| {
                r.ports
                    .iter()
                    .any(|p| p.service.as_deref() == Some(service.as_str()))
            }),
            Self::HasOpenPorts => results
                .iter()
                .any(|r| r.ports.iter().any(|p| matches!(p.state, PortState::Open))),
            Self::HasResults => !results.is_empty(),
            Self::OsDetected(os_name) => results.iter().any(|r| {
                r.os.as_ref()
                    .is_some_and(|os| os.name.to_lowercase().contains(&os_name.to_lowercase()))
            }),
        }
    }
}

/// A single step in a scan chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStep {
    /// Step name
    pub name: String,
    /// Scan type to perform
    pub scan_type: ScanType,
    /// Ports to scan (empty means use defaults)
    pub ports: Vec<u16>,
    /// Condition for execution
    pub condition: StepCondition,
    /// Whether to enable service detection
    pub service_detection: bool,
    /// Whether to enable OS detection
    pub os_detection: bool,
    /// Description of this step
    pub description: String,
}

impl ScanStep {
    /// Create a new scan step
    pub fn new(name: &str, scan_type: ScanType) -> Self {
        Self {
            name: name.to_string(),
            scan_type,
            ports: Vec::new(),
            condition: StepCondition::Always,
            service_detection: false,
            os_detection: false,
            description: String::new(),
        }
    }

    /// Set ports to scan
    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports = ports;
        self
    }

    /// Set execution condition
    pub fn with_condition(mut self, condition: StepCondition) -> Self {
        self.condition = condition;
        self
    }

    /// Enable service detection
    pub const fn with_service_detection(mut self) -> Self {
        self.service_detection = true;
        self
    }

    /// Enable OS detection
    pub const fn with_os_detection(mut self) -> Self {
        self.os_detection = true;
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
}

/// A chain of scans that execute sequentially
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanChain {
    /// Chain name
    pub name: String,
    /// Description
    pub description: String,
    /// Steps in execution order
    pub steps: Vec<ScanStep>,
    /// Results from each step (keyed by step name)
    #[serde(skip)]
    pub step_results: HashMap<String, Vec<ScanResult>>,
    /// Steps that were skipped (condition not met)
    pub skipped_steps: Vec<String>,
}

impl ScanChain {
    /// Create a new scan chain
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            steps: Vec::new(),
            step_results: HashMap::new(),
            skipped_steps: Vec::new(),
        }
    }

    /// Create a builder for constructing scan chains
    pub fn builder(name: &str) -> ScanChainBuilder {
        ScanChainBuilder::new(name)
    }

    /// Add a step to the chain
    pub fn add_step(&mut self, step: ScanStep) {
        self.steps.push(step);
    }

    /// Get the number of steps
    pub const fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get results from a specific step
    pub fn results_for_step(&self, step_name: &str) -> Option<&Vec<ScanResult>> {
        self.step_results.get(step_name)
    }

    /// Get all results from all steps combined
    pub fn all_results(&self) -> Vec<&ScanResult> {
        self.step_results.values().flat_map(|v| v.iter()).collect()
    }

    /// Evaluate the chain, determining which steps should execute
    /// based on results accumulated so far.
    ///
    /// Returns the indices of steps that should execute.
    #[instrument(skip(self), fields(chain = %self.name))]
    pub fn evaluate_steps(&self) -> Vec<usize> {
        let mut executable = Vec::new();
        let mut accumulated_results: Vec<ScanResult> = Vec::new();

        for (idx, step) in self.steps.iter().enumerate() {
            let should_run = if idx == 0 {
                // First step always evaluates against empty results (Always condition)
                step.condition.evaluate(&[])
            } else {
                step.condition.evaluate(&accumulated_results)
            };

            if should_run {
                debug!(step = %step.name, idx = idx, "Step will execute");
                executable.push(idx);
                // Add any existing results for this step
                if let Some(results) = self.step_results.get(&step.name) {
                    accumulated_results.extend(results.iter().cloned());
                }
            } else {
                debug!(step = %step.name, idx = idx, "Step skipped (condition not met)");
            }
        }

        info!(
            chain = %self.name,
            total_steps = self.steps.len(),
            executable = executable.len(),
            "Chain evaluation complete"
        );

        executable
    }

    /// Record results for a step
    pub fn record_results(&mut self, step_name: &str, results: Vec<ScanResult>) {
        self.step_results.insert(step_name.to_string(), results);
    }

    /// Mark a step as skipped
    pub fn mark_skipped(&mut self, step_name: &str) {
        self.skipped_steps.push(step_name.to_string());
    }
}

/// Builder for constructing scan chains ergonomically
pub struct ScanChainBuilder {
    name: String,
    description: String,
    steps: Vec<ScanStep>,
}

impl ScanChainBuilder {
    /// Create a new builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            steps: Vec::new(),
        }
    }

    /// Set description
    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Add a step
    pub fn step(mut self, step: ScanStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Build the scan chain
    pub fn build(self) -> ScanChain {
        let mut chain = ScanChain::new(&self.name);
        chain.description = self.description;
        chain.steps = self.steps;
        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::{OsInfo, PortResult, Protocol, ScanResult, Target};
    use std::time::Duration;

    fn make_result(host: &str, ports: Vec<(u16, PortState, Option<&str>)>) -> ScanResult {
        ScanResult {
            host: host.to_string(),
            ip: Target::Ip(host.parse().unwrap()),
            hostname: None,
            ports: ports
                .into_iter()
                .map(|(port, state, service)| PortResult {
                    port,
                    protocol: Protocol::Tcp,
                    state,
                    service: service.map(String::from),
                    version: None,
                    banner: None,
                })
                .collect(),
            os: None,
            scan_time: Duration::from_millis(100),
        }
    }

    #[test]
    fn test_step_condition_always() {
        assert!(StepCondition::Always.evaluate(&[]));
    }

    #[test]
    fn test_step_condition_port_open() {
        let results = vec![make_result("10.0.0.1", vec![(80, PortState::Open, None)])];
        assert!(StepCondition::PortOpen(80).evaluate(&results));
        assert!(!StepCondition::PortOpen(443).evaluate(&results));
    }

    #[test]
    fn test_step_condition_service_detected() {
        let results = vec![make_result(
            "10.0.0.1",
            vec![(22, PortState::Open, Some("ssh"))],
        )];
        assert!(StepCondition::ServiceDetected("ssh".to_string()).evaluate(&results));
        assert!(!StepCondition::ServiceDetected("http".to_string()).evaluate(&results));
    }

    #[test]
    fn test_step_condition_has_open_ports() {
        let results_open = vec![make_result("10.0.0.1", vec![(80, PortState::Open, None)])];
        assert!(StepCondition::HasOpenPorts.evaluate(&results_open));

        let results_closed = vec![make_result("10.0.0.1", vec![(80, PortState::Closed, None)])];
        assert!(!StepCondition::HasOpenPorts.evaluate(&results_closed));
    }

    #[test]
    fn test_step_condition_has_results() {
        assert!(!StepCondition::HasResults.evaluate(&[]));
        let results = vec![make_result("10.0.0.1", vec![])];
        assert!(StepCondition::HasResults.evaluate(&results));
    }

    #[test]
    fn test_step_condition_os_detected() {
        let mut result = make_result("10.0.0.1", vec![]);
        result.os = Some(OsInfo {
            name: "Linux".to_string(),
            version: Some("5.15".to_string()),
            confidence: 90,
        });
        let results = vec![result];
        assert!(StepCondition::OsDetected("linux".to_string()).evaluate(&results));
        assert!(!StepCondition::OsDetected("windows".to_string()).evaluate(&results));
    }

    #[test]
    fn test_step_condition_custom() {
        assert!(StepCondition::Custom("my-cond".to_string()).evaluate(&[]));
    }

    #[test]
    fn test_scan_step_builder() {
        let step = ScanStep::new("syn-scan", ScanType::Syn)
            .with_ports(vec![22, 80, 443])
            .with_condition(StepCondition::Always)
            .with_service_detection()
            .with_os_detection()
            .with_description("Initial SYN scan");

        assert_eq!(step.name, "syn-scan");
        assert_eq!(step.ports, vec![22, 80, 443]);
        assert!(step.service_detection);
        assert!(step.os_detection);
        assert_eq!(step.description, "Initial SYN scan");
    }

    #[test]
    fn test_scan_chain_new() {
        let chain = ScanChain::new("test-chain");
        assert_eq!(chain.name, "test-chain");
        assert_eq!(chain.step_count(), 0);
    }

    #[test]
    fn test_scan_chain_add_steps() {
        let mut chain = ScanChain::new("chain");
        chain.add_step(ScanStep::new("step-1", ScanType::Connect));
        chain.add_step(ScanStep::new("step-2", ScanType::Syn));
        assert_eq!(chain.step_count(), 2);
    }

    #[test]
    fn test_scan_chain_builder() {
        let chain = ScanChain::builder("recon")
            .description("Reconnaissance chain")
            .step(ScanStep::new("discovery", ScanType::Connect).with_ports(vec![80, 443]))
            .step(
                ScanStep::new("deep-scan", ScanType::Syn)
                    .with_condition(StepCondition::HasOpenPorts),
            )
            .build();

        assert_eq!(chain.name, "recon");
        assert_eq!(chain.step_count(), 2);
    }

    #[test]
    fn test_scan_chain_evaluate_steps() {
        let chain = ScanChain::builder("test")
            .step(ScanStep::new("first", ScanType::Connect))
            .step(ScanStep::new("second", ScanType::Syn).with_condition(StepCondition::HasResults))
            .build();

        // No results yet, first step runs but second won't (no results to evaluate)
        let executable = chain.evaluate_steps();
        assert_eq!(executable.len(), 1);
        assert_eq!(executable[0], 0);
    }

    #[test]
    fn test_scan_chain_record_and_query_results() {
        let mut chain = ScanChain::new("test");
        chain.add_step(ScanStep::new("step-1", ScanType::Connect));

        let results = vec![make_result("10.0.0.1", vec![(80, PortState::Open, None)])];
        chain.record_results("step-1", results);

        assert!(chain.results_for_step("step-1").is_some());
        assert_eq!(chain.results_for_step("step-1").map(Vec::len), Some(1));
        assert!(chain.results_for_step("step-2").is_none());
    }

    #[test]
    fn test_scan_chain_all_results() {
        let mut chain = ScanChain::new("test");
        chain.record_results(
            "step-1",
            vec![make_result("10.0.0.1", vec![(80, PortState::Open, None)])],
        );
        chain.record_results(
            "step-2",
            vec![make_result("10.0.0.2", vec![(22, PortState::Open, None)])],
        );

        let all = chain.all_results();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_scan_chain_mark_skipped() {
        let mut chain = ScanChain::new("test");
        chain.mark_skipped("step-2");
        assert_eq!(chain.skipped_steps, vec!["step-2"]);
    }

    #[test]
    fn test_chain_serialization() {
        let chain = ScanChain::builder("test")
            .description("Test chain")
            .step(ScanStep::new("step-1", ScanType::Connect).with_ports(vec![80]))
            .build();

        let json = serde_json::to_string(&chain).unwrap();
        let deserialized: ScanChain = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.step_count(), 1);
    }
}
