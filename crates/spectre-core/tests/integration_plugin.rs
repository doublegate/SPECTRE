//! Integration tests -- plugin system
#![allow(clippy::disallowed_methods)]

use spectre_core::plugin::{
    HookContext, HookEvent, HookManager, HookRegistration, PluginRegistry, PluginTemplateGenerator,
    PluginTemplateType, RegistryEntry,
};
use std::path::PathBuf;

fn make_entry(name: &str, version: &str, deps: Vec<&str>) -> RegistryEntry {
    RegistryEntry {
        name: name.to_string(),
        version: version.to_string(),
        description: format!("{} plugin", name),
        author: "Test".to_string(),
        path: PathBuf::from(format!("/plugins/{}", name)),
        enabled: true,
        tags: vec!["test".to_string()],
        dependencies: deps.into_iter().map(|s| s.to_string()).collect(),
    }
}

#[test]
fn test_plugin_registry_lifecycle() {
    let mut registry = PluginRegistry::new();

    registry.register(make_entry("base-lib", "1.0.0", vec![]));
    registry.register(make_entry("scanner-ext", "2.0.0", vec!["base-lib"]));
    registry.register(make_entry("reporter-ext", "1.5.0", vec!["base-lib"]));
    assert_eq!(registry.count(), 3);

    let results = registry.search_by_tag("test");
    assert_eq!(results.len(), 3);

    let results = registry.search_by_name("scanner");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "scanner-ext");

    let order = registry.resolve_load_order().unwrap();
    let base_pos = order.iter().position(|&n| n == "base-lib").unwrap();
    let scanner_pos = order.iter().position(|&n| n == "scanner-ext").unwrap();
    let reporter_pos = order.iter().position(|&n| n == "reporter-ext").unwrap();
    assert!(base_pos < scanner_pos);
    assert!(base_pos < reporter_pos);

    let missing = registry.check_dependencies("scanner-ext");
    assert!(missing.is_empty());

    assert!(registry.set_enabled("scanner-ext", false));
    assert_eq!(registry.list_enabled().len(), 2);

    assert!(registry.unregister("reporter-ext"));
    assert_eq!(registry.count(), 2);
}

#[test]
fn test_plugin_registry_persistence() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("registry.json");

    let mut registry = PluginRegistry::new();
    registry.register(make_entry("plugin-a", "1.0.0", vec![]));
    registry.register(make_entry("plugin-b", "2.0.0", vec!["plugin-a"]));
    registry.save(&path).unwrap();

    let loaded = PluginRegistry::load(&path).unwrap();
    assert_eq!(loaded.count(), 2);
    assert!(loaded.get("plugin-a").is_some());
    assert!(loaded.get("plugin-b").is_some());

    let missing = loaded.check_dependencies("plugin-b");
    assert!(missing.is_empty());
}

#[test]
fn test_hook_system_integration() {
    let mut manager = HookManager::new();

    manager.register(
        HookRegistration::new("scanner-plugin", HookEvent::PreScan, "on_pre_scan")
            .with_priority(10),
    );
    manager.register(
        HookRegistration::new("scanner-plugin", HookEvent::PostScan, "on_post_scan")
            .with_priority(10),
    );
    manager.register(
        HookRegistration::new("reporter-plugin", HookEvent::PostScan, "collect_results")
            .with_priority(20),
    );
    manager.register(HookRegistration::new(
        "reporter-plugin",
        HookEvent::PreReport,
        "prepare_data",
    ));

    assert_eq!(manager.hook_count(), 4);

    let pre_scan_hooks = manager.get_hooks(HookEvent::PreScan);
    assert_eq!(pre_scan_hooks.len(), 1);

    let post_scan_hooks = manager.get_hooks(HookEvent::PostScan);
    assert_eq!(post_scan_hooks.len(), 2);
    assert_eq!(post_scan_hooks[0].plugin_name, "scanner-plugin");
    assert_eq!(post_scan_hooks[1].plugin_name, "reporter-plugin");

    manager.set_plugin_enabled("scanner-plugin", false);
    let pre_scan_hooks = manager.get_hooks(HookEvent::PreScan);
    assert!(pre_scan_hooks.is_empty());

    manager.unregister_plugin("scanner-plugin");
    assert_eq!(manager.hook_count(), 2);
}

#[test]
fn test_hook_context_data_flow() {
    let ctx = HookContext::new(HookEvent::PostScan)
        .with_data("target", "10.0.0.0/24")
        .with_data("hosts_found", "15");

    assert_eq!(ctx.event, HookEvent::PostScan);
    assert_eq!(ctx.get("target"), Some("10.0.0.0/24"));
    assert_eq!(ctx.get("hosts_found"), Some("15"));

    let json = serde_json::to_string(&ctx).unwrap();
    let loaded: HookContext = serde_json::from_str(&json).unwrap();
    assert_eq!(loaded.event, HookEvent::PostScan);
    assert_eq!(loaded.get("target"), Some("10.0.0.0/24"));
}

#[test]
fn test_plugin_template_generation() {
    let dir = tempfile::tempdir().unwrap();

    for template_type in PluginTemplateType::all() {
        let name = format!("test-{}", template_type);
        let path =
            PluginTemplateGenerator::generate(&name, template_type, "Test Author", dir.path())
                .unwrap();

        assert!(path.join("plugin.toml").exists());
        assert!(path.join("init.lua").exists());

        let manifest = std::fs::read_to_string(path.join("plugin.toml")).unwrap();
        assert!(manifest.contains(&name));
        assert!(manifest.contains("Test Author"));

        let script = std::fs::read_to_string(path.join("init.lua")).unwrap();
        assert!(script.contains("plugin_run"));
    }
}

#[test]
fn test_version_comparison() {
    use std::cmp::Ordering;
    assert_eq!(
        PluginRegistry::compare_versions("1.0.0", "1.0.0"),
        Ordering::Equal
    );
    assert_eq!(
        PluginRegistry::compare_versions("1.0.0", "2.0.0"),
        Ordering::Less
    );
    assert_eq!(
        PluginRegistry::compare_versions("2.1.0", "2.0.5"),
        Ordering::Greater
    );
    assert_eq!(
        PluginRegistry::compare_versions("1.9.0", "1.10.0"),
        Ordering::Less
    );
}
