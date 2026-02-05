//! Advanced scan orchestration module
//!
//! Provides scan chaining, conditional execution, templates, scheduling,
//! profiles, adaptive timing, and checkpoint/resume capabilities.

mod chain;
mod checkpoint;
mod profile;
mod schedule;
mod template;
mod timing;

pub use chain::{ScanChain, ScanChainBuilder, ScanStep, StepCondition};
pub use checkpoint::{Checkpoint, CheckpointStore};
pub use profile::{ScanProfile, ScanProfileStore};
pub use schedule::{ScanScheduler, ScheduleEntry, ScheduleExpression};
pub use template::{ScanTemplate, TemplateLibrary};
pub use timing::{AdaptiveTiming, TimingMetrics};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_reexports() {
        // Verify that all public types are accessible
        let _chain = ScanChain::new("test-chain");
        let _template = ScanTemplate::quick_recon();
        let _profile = ScanProfile::new("default", crate::scan::ScanType::Connect);
    }
}
