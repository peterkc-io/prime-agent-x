# Upstream seams

This ledger compares the working tree or selected commit with `upstream/main`.
The foundation base is `c24ac227f11f552ed1d3fa8ebc7d916937d7f6bc`.

```sh
python3 scripts/agx/check_seams.py
python3 scripts/agx/check_seams.py --revision HEAD
python3 scripts/agx/test_check_seams.py
```

The checker rejects missing rows and stale unchanged-file rows. New fork-owned
files are outside the upstream seam set. Verification names below identify the
checks for each seam; the worker report carries observed results. Linux CI is
lead-owned. Windows runtime behavior is not checked on the macOS host.

The old three installer-download scenarios are unreachable in agx and are
replaced by three no-network refusal scenarios through Cargo's existing test
target. No other test is deleted or newly ignored. The two Linux-only test
files retain their Linux coverage. Their macOS code exclusions are distinct
from accepted flakes. The prerequisite platform adjustments are intentional.

Open upstream PR #3339 overlaps another part of `interactive_daemon_e2e.rs`,
not the timeout seam. It remains a future merge-conflict risk.

| Upstream file | Reason | Verification |
| --- | --- | --- |
| `Cargo.lock` | Record the local agx-identity crate dependency. | Locked workspace build. |
| `Cargo.toml` | Add the fork-owned identity workspace crate. | Workspace fmt, clippy, and build. |
| `crates/pa-cli/Cargo.toml` | Name the binary agx; replace three retired installer scenarios with fork update refusals. | Three no-network update tests; workspace target checks. |
| `crates/pa-cli/src/client_traces.rs` | Report trace sharing unsupported and off. | Four client-trace tests and public no-request trace test; legacy protocol coverage retained. |
| `crates/pa-cli/src/client_update.rs` | Reject TUI installer execution before side effects. | Fork update refusal and actual TUI error-exit scenario. |
| `crates/pa-cli/src/config.rs` | Name the app agx; reject resolved upstream socket paths. | Four config tests, including both namespaces and all sources. |
| `crates/pa-cli/src/daemon_client.rs` | macOS setsockopt after peer close returns EINVAL; set READ_POLL once. | Four daemon-client tests; closing-reason test 50/50. |
| `crates/pa-cli/src/daemon_command.rs` | Apply socket precedence before guarded resolution. | CLI daemon parser tests and explicit socket rejection scenario. |
| `crates/pa-cli/src/daemon_mode.rs` | Propagate socket rejection before starting a supervisor. | Config tests and live socket coexistence scenario. |
| `crates/pa-cli/src/installer_update.rs` | Reject the native installer runner before networking. | Three fork update refusal tests. |
| `crates/pa-cli/src/interactive_mode.rs` | Propagate guarded socket resolution. | Config tests and live socket coexistence scenario. |
| `crates/pa-cli/src/interactive_mode/daemon.rs` | Reject upstream sockets before stale-daemon checks or replacement. | Socket unit tests and live coexistence scenario. |
| `crates/pa-cli/src/lib.rs` | Prefix version output with agx. | Local install version check. |
| `crates/pa-cli/src/main.rs` | Initialize fork lineage and default venv override before threading or dispatch. | Seven identity tests; bare release runtime and native venv override regression. |
| `crates/pa-cli/src/print_runtime.rs` | Propagate socket rejection before opening or leasing a session. | Socket source tests and print-session fixture. |
| `crates/pa-cli/src/public_command.rs` | Block update and alias before any installer requests. | Three fork update refusal tests. |
| `crates/pa-cli/tests/acp_config_option_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/acp_mode_e2e.rs` | Use agx executable and default kernel discovery; retain ACP protocol identity. | ACP targets in Linux CI (lead-owned). |
| `crates/pa-cli/tests/agents_view_anchor_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/agents_view_flash_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/agents_view_refusal_panel_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/agents_view_reply_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/agents_view_search_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/agents_view_subagents_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/chat_agents_chat_roundtrip_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/clipboard_auth_commands_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/continue_guard_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/daemon_commands_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/daemon_discovery_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/differential_cli.rs` | Normalize only the fork executable/version identity in the existing comparison. | Differential CLI corpus with its configured upstream oracle. |
| `crates/pa-cli/tests/dock_scope_exit_focus_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/eval_composition_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/export_live_differential.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/export_share_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/interactive_daemon_e2e.rs` | Use agx executable; Linux-only test code, upstream CI is Linux-only. | Linux integration suite; six termios tests are compiled out on macOS. |
| `crates/pa-cli/tests/interactive_fork_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/kitty_early_typing_e2e.rs` | macOS portability; upstream CI is Linux-only. | Native focused terminal checks; Linux baseline comparison (lead-owned). |
| `crates/pa-cli/tests/kitty_exit_routes_e2e/main.rs` | macOS portability; upstream CI is Linux-only. | Native focused terminal checks; Linux baseline comparison (lead-owned). |
| `crates/pa-cli/tests/kitty_release_handoff_e2e.rs` | macOS portability; upstream CI is Linux-only. | Native focused terminal checks; Linux baseline comparison (lead-owned). |
| `crates/pa-cli/tests/kitty_verdict_time_e2e.rs` | macOS portability; upstream CI is Linux-only. | Native focused terminal checks; Linux baseline comparison (lead-owned). |
| `crates/pa-cli/tests/lock_compat_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/mcp_view_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/package_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/package_resources_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/packaged_layout_e2e.rs` | Use Cargo executable agx and its version prefix, preserving hostile-env isolation. | Exact hostile packaged-layout regression and lead-owned Linux CI. |
| `crates/pa-cli/tests/print_runtime_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/rpc_mode_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/slow_drain_exit_guard_e2e.rs` | macOS portability; upstream CI is Linux-only. | Native focused terminal checks; Linux baseline comparison (lead-owned). |
| `crates/pa-cli/tests/subagent_panel_nav_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/subagent_panel_nested_count_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-cli/tests/telemetry_command_e2e.rs` | Use Cargo executable agx; retain upstream debug telemetry assertions. | Original debug command integration test and exact telemetry regressions. |
| `crates/pa-cli/tests/terminal_state_differential_e2e/harness.rs` | macOS portability; upstream CI is Linux-only. | Native focused terminal checks; Linux baseline comparison (lead-owned). |
| `crates/pa-cli/tests/termios_process_exit_e2e.rs` | Linux-only test code; upstream CI is Linux-only. | Linux integration suite (lead-owned). |
| `crates/pa-cli/tests/view_switch_latency_e2e.rs` | Use Cargo executable agx in the existing fixture. | CLI integration target compilation and Linux suites (lead-owned). |
| `crates/pa-core/src/agent_traces.rs` | Add the fork-owned public upload guard test module. | Recording test has a reachable HTTP positive control. |
| `crates/pa-core/src/agent_traces/tests.rs` | Keep legacy protocol tests on private carried upstream machinery. | Legacy trace unit tests and separate public guard test. |
| `crates/pa-core/src/agent_traces/upload.rs` | Disable every public single-file upload before requests, including forced options. | Recording fake returns Disabled with zero requests. |
| `crates/pa-core/src/agent_traces/upload_all.rs` | Disable public upload-all before payload collection or requests; retain private protocol coverage. | Recording fake for upload-all and legacy protocol tests. |
| `crates/pa-core/src/kernel/bootstrap/venv/layout.rs` | Keep the writable-dir fallback in the fork namespace; the entry point supplies the normal default. | Unchanged venv_dir_honors_override test; native library suite. |
| `crates/pa-core/src/kernel/manager/startup.rs` | Remove internal role variables at the kernel spawn. | Actual session ipython environment inspection. |
| `crates/pa-core/src/session_engine/telemetry/status.rs` | Force release switch off before env/settings; endpoint None in every build; retain debug semantics. | Pure release-rule unit test, unchanged debug tests, and opt-in installed release scenario. |
| `crates/pa-core/src/session_engine/telemetry/tests.rs` | Serialize the unchanged mirror test with the approved ENV_MUTEX prerequisite. | Full native library tests; original mirror and switch assertions retained. |
| `crates/pa-core/src/tools/bash_local.rs` | Remove internal role variables immediately before bash spawn. | Actual BashOperations spawn and session kernel bash-helper environment inspection. |
| `crates/pa-core/tests/abort_kernel_cell.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_capture_freshness.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_lifecycle.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_prewarm.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_python_skills_unavailable.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_restore_guards.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_snapshot_resume.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_stop_revive.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/kernel_teardown.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-core/tests/turn_boundary_kernel.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/src/agent_engine/tests/abort.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/src/platform/paths.rs` | Separate Unix directory and Windows pipe namespaces. | Default path inspection and socket unit tests. |
| `crates/pa-daemon/src/socket.rs` | Refuse upstream sockets before bind, unlink, or stale cleanup. | Socket guard units and live coexistence scenario. |
| `crates/pa-daemon/tests/abort_kernel_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/agent_family_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/herdr_pane_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/idle_passivation_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/kernel_dispose_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/mcp_catalog_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/mcp_product_path_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/peer_messaging_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/readoption_wake_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/replacement_kernel_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/rlm_children_parent_death_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/rlm_children_replacement_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/rlm_quiescence_barrier_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-daemon/tests/steer_boundary_batch_e2e.rs` | Discover the fork default kernel venv without adding skips. | Linux kernel integration suites after agx bootstrap (lead-owned). |
| `crates/pa-tui/src/interactive/run.rs` | Close the TUI only for a typed disabled-update rejection; keep ordinary prompt errors soft. | Actual /update error exit and terminal restoration scenario. |
| `crates/pa-tui/src/session_ui/commands.rs` | Raise the typed update rejection with local install instructions. | TUI update dispatch tests and actual error-exit scenario. |
| `crates/pa-tui/src/session_ui/share.rs` | Report disabled trace sharing before login or upload. | TUI trace dispatch and recording public trace tests. |
| `crates/pa-tui/src/traces.rs` | Expose unsupported trace capability with an upstream-compatible default. | TUI trace tests with carried generic fixtures. |
| `crates/pa-tui/src/update_command.rs` | Expose update unavailability with an upstream-compatible default. | TUI update tests and actual disabled-command exit. |
| `crates/pa-types/Cargo.toml` | Depend on the fork-owned identity helpers. | Locked workspace build. |
| `crates/pa-types/src/lib.rs` | Expose one fork identity helper module to current callers. | Workspace target compilation and helper unit tests. |
| `crates/pa-types/src/platform/transport.rs` | Refuse upstream endpoints at six connect/bind boundaries. | Socket guard tests; native transport fixtures. |
| `scripts/package_release.py` | Add a binary-name option while preserving upstream defaults. | Local packer and atomic installer tests. |
