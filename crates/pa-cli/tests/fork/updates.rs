//! pa-x never contacts a release manifest or installer, including update aliases.
use std::net::TcpListener;
use std::process::Command;

fn disabled(args: &[&str]) {
    let root = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let sentinel = root.path().join("upstream-install");
    std::fs::write(&sentinel, "unchanged").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_pa-x"))
        .args(args)
        .env_clear()
        .env("HOME", root.path())
        .env("TMPDIR", "/private/tmp")
        .env("TZ", "UTC")
        .env(
            "PRIME_AGENT_RUST_INSTALLER_URL",
            format!("{base}/install.sh"),
        )
        .env("PRIME_AGENT_RUST_DOWNLOAD_BASE", &base)
        .env("PRIME_AGENT_RUST_INSTALL_DIR", &sentinel)
        .current_dir(root.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let text = String::from_utf8_lossy(&output.stderr);
    assert!(text.contains("scripts/pa-x/install-local.sh"), "{text}");
    assert!(text.contains("docs/fork/INSTALL.md"), "{text}");
    assert_eq!(std::fs::read_to_string(sentinel).unwrap(), "unchanged");
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn update_never_fetches_or_installs() {
    disabled(&["update", "--nightly"]);
}
#[test]
fn update_check_never_fetches_a_manifest() {
    disabled(&["update", "--check"]);
}
#[test]
fn update_version_alias_never_fetches_a_manifest() {
    disabled(&["update", "--version", "--nightly"]);
}
