use super::*;
use std::process::Command;

#[test]
fn socket_sources_refuse_both_upstream_namespaces() {
    let root = tempfile::tempdir().unwrap();
    for namespace in ["prime-agent-user", "prime-agent-rust-501"] {
        for source in ["flag", "env", "default"] {
            if cfg!(windows) && source == "default" {
                continue; // Windows defaults are named pipes, not TMPDIR paths.
            }
            let socket = root.path().join(namespace).join("daemon.sock");
            let mut command = Command::new(std::env::current_exe().unwrap());
            command
                .args(["--exact", "config::fork_tests::socket_source_child"])
                .env("AGX_SOCKET_TEST_SOURCE", source)
                .env("AGX_SOCKET_TEST_PATH", &socket)
                .env_remove(ENV_DAEMON_SOCKET);
            if source == "env" {
                command.env(ENV_DAEMON_SOCKET, &socket);
            } else if source == "default" {
                command.env("TMPDIR", socket.parent().unwrap());
            }
            let output = command.output().unwrap();
            assert!(output.status.success(), "{namespace}/{source}: {output:?}");
        }
    }
}

#[test]
fn socket_source_child() {
    let Ok(source) = std::env::var("AGX_SOCKET_TEST_SOURCE") else {
        return;
    };
    let socket = std::env::var("AGX_SOCKET_TEST_PATH").unwrap();
    let flag = (source == "flag").then_some(socket.as_str());
    let error = resolve_daemon_socket_path(flag).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
    assert!(error.to_string().contains("agx refuses upstream socket:"));
    if source == "env" {
        assert!(resolve_daemon_socket_path(Some("/tmp/agx-custom.sock")).is_ok());
    }
}
