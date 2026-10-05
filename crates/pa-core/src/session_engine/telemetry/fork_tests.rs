//! The release rule is independent of upstream env and settings precedence.
use super::*;

#[test]
fn release_switch_is_always_off_while_debug_uses_upstream_precedence() {
    let release = fork_switch(false).unwrap();
    assert_eq!(release, TelemetrySwitch::ForkDisabled);
    assert!(!release.enabled());
    assert_eq!(release.reason(), "disabled by pa-x");
    assert_eq!(fork_switch(true), None);
    assert_eq!(telemetry_endpoint(), None);
}
