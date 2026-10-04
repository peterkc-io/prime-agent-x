//! Fork-owned process and socket isolation. Shared settings and sessions stay upstream-compatible.

use std::ffi::OsStr;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

pub const TRACES_DISABLED: bool = true;
pub const UPDATES_DISABLED: bool = true;

pub const UPDATE_INSTRUCTIONS: &str = "Automatic updates are disabled for pa-x.\nUpdate from a verified checkout:\n  scripts/pa-x/install-local.sh\nSee docs/fork/INSTALL.md.\n";

/// A disabled update closes the TUI after its normal terminal cleanup.
#[derive(Debug)]
pub struct UpdateUnavailable(pub &'static str);

impl std::fmt::Display for UpdateUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}
impl std::error::Error for UpdateUnavailable {}

fn internal_key(key: &OsStr) -> bool {
    let prefix = b"PRIME_AGENT_INTERNAL_";
    let bytes = key.as_encoded_bytes();
    if cfg!(windows) {
        bytes
            .get(..prefix.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
    } else {
        bytes.starts_with(prefix)
    }
}

fn upstream_environment_key(key: &OsStr) -> bool {
    [
        "PRIME_AGENT_DAEMON_SOCKET",
        "PRIME_AGENT_KERNEL_VENV",
        "PRIME_AGENT_KERNEL_OWNER_PID",
        "PI_PACKAGE_DIR",
    ]
    .iter()
    .any(|name| {
        if cfg!(windows) {
            key.as_encoded_bytes().eq_ignore_ascii_case(name.as_bytes())
        } else {
            key == OsStr::new(name)
        }
    })
}

/// Call before any threads start. Marked fork children keep their inherited environment.
pub fn initialize_process() {
    if std::env::var_os("PA_X_PROCESS").is_none() {
        let keys: Vec<_> = std::env::vars_os()
            .map(|(key, _)| key)
            .filter(|key| internal_key(key) || upstream_environment_key(key))
            .collect();
        for key in keys {
            std::env::remove_var(key);
        }
    }
    std::env::set_var("PA_X_PROCESS", "1");
}

/// Session shell and kernel processes cannot inherit internal daemon roles.
pub fn strip_internal_environment(command: &mut Command) {
    let keys: Vec<_> = std::env::vars_os()
        .map(|(key, _)| key)
        .chain(command.get_envs().map(|(key, _)| key.to_owned()))
        .filter(|key| internal_key(key))
        .collect();
    for key in keys {
        command.env_remove(key);
    }
}

fn upstream_directory(name: &OsStr) -> bool {
    let Some(name) = name.to_str() else {
        return false;
    };
    let folded;
    let name = if cfg!(windows) {
        folded = name.to_ascii_lowercase();
        folded.as_str()
    } else {
        name
    };
    for prefix in ["prime-agent-", "prime-agent-rust-"] {
        if let Some(suffix) = name.strip_prefix(prefix) {
            if (prefix == "prime-agent-" && suffix == "user")
                || (!suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit()))
            {
                return true;
            }
        }
    }
    false
}

fn resolved_path(path: &Path) -> io::Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut ancestor = absolute.clone();
    let mut tail = Vec::new();
    let mut resolved = loop {
        match ancestor.canonicalize() {
            Ok(resolved) => break resolved,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let Some(name) = ancestor.file_name() else {
                    return Err(error);
                };
                tail.push(name.to_owned());
                ancestor.pop();
            }
            Err(error) => return Err(error),
        }
    };
    for name in tail.into_iter().rev() {
        resolved.push(name);
    }
    let mut normalized = PathBuf::new();
    for component in resolved.components() {
        match component {
            Component::ParentDir => {
                normalized.pop();
            }
            Component::CurDir => {}
            other => normalized.push(other.as_os_str()),
        }
    }
    Ok(normalized)
}

/// Refuse an upstream endpoint before connecting, stopping or replacing it.
/// Existing symlink parents are resolved even when the socket does not yet exist.
pub fn reject_upstream_socket(path: &Path) -> io::Result<()> {
    #[cfg(windows)]
    if path
        .to_string_lossy()
        .to_ascii_lowercase()
        .starts_with(r"\\.\pipe\")
    {
        return if path
            .to_string_lossy()
            .to_ascii_lowercase()
            .starts_with(r"\\.\pipe\prime-agent-")
        {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("pa-x refuses upstream socket: {}", path.display()),
            ))
        } else {
            Ok(())
        };
    }
    let resolved = resolved_path(path)?;
    if resolved
        .components()
        .any(|component| upstream_directory(component.as_os_str()))
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("pa-x refuses upstream socket: {}", resolved.display()),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const UPSTREAM_ENV: [&str; 6] = [
        "PRIME_AGENT_DAEMON_SOCKET",
        "PRIME_AGENT_KERNEL_VENV",
        "PRIME_AGENT_KERNEL_OWNER_PID",
        "PI_PACKAGE_DIR",
        "PRIME_AGENT_INTERNAL_TEST_A",
        "PRIME_AGENT_INTERNAL_TEST_B",
    ];

    #[test]
    fn environment_and_directory_case_matching_follows_the_platform() {
        assert!(upstream_environment_key(OsStr::new("PI_PACKAGE_DIR")));
        assert_eq!(
            upstream_environment_key(OsStr::new("pi_package_dir")),
            cfg!(windows)
        );
        assert_eq!(
            internal_key(OsStr::new("prime_agent_internal_TEST")),
            cfg!(windows)
        );
        assert_eq!(
            upstream_directory(OsStr::new("PRIME-AGENT-user")),
            cfg!(windows)
        );
    }

    #[test]
    fn reserved_socket_directories_include_user_and_both_numeric_forms() {
        for name in [
            "prime-agent-user",
            "prime-agent-0",
            "prime-agent-501",
            "prime-agent-rust-501",
        ] {
            assert!(upstream_directory(OsStr::new(name)), "{name}");
        }
        for name in [
            "pa-x-user",
            "pa-x-501",
            "prime-agent",
            "prime-agent-rust-user",
            "prime-agent-custom",
            "prime-agent-501-extra",
        ] {
            assert!(!upstream_directory(OsStr::new(name)), "{name}");
        }
    }

    #[test]
    fn rejects_reserved_parents_and_accepts_fork_or_custom_paths() {
        let root = tempfile::tempdir().unwrap();
        for name in [
            "prime-agent-user",
            "prime-agent-501",
            "prime-agent-rust-501",
        ] {
            let path = root.path().join(name).join("daemon.sock");
            let error = reject_upstream_socket(&path).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        }
        assert!(reject_upstream_socket(&root.path().join("pa-x-user/daemon.sock")).is_ok());
        assert!(reject_upstream_socket(&root.path().join("custom/daemon.sock")).is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn rejects_existing_symlink_parents_before_the_socket_exists() {
        let root = tempfile::tempdir().unwrap();
        let upstream = root.path().join("prime-agent-user");
        std::fs::create_dir(&upstream).unwrap();
        let alias = root.path().join("alias");
        std::os::unix::fs::symlink(upstream, &alias).unwrap();
        assert_eq!(
            reject_upstream_socket(&alias.join("daemon.sock"))
                .unwrap_err()
                .kind(),
            io::ErrorKind::PermissionDenied
        );
    }

    #[test]
    fn startup_environment_is_cleared_only_for_unmarked_processes() {
        for marked in [false, true] {
            let mut command = Command::new(std::env::current_exe().unwrap());
            command.args(["--exact", "tests::startup_child", "--nocapture"]);
            command.env(
                "PA_X_IDENTITY_TEST_CHILD",
                if marked { "marked" } else { "unmarked" },
            );
            command.env_remove("PA_X_PROCESS");
            if marked {
                command.env("PA_X_PROCESS", "1");
            }
            for key in UPSTREAM_ENV {
                command.env(key, "upstream");
            }
            command.env("PA_X_KEEP", "retained");
            assert!(command.status().unwrap().success());
        }
    }

    #[test]
    fn startup_child() {
        let Ok(case) = std::env::var("PA_X_IDENTITY_TEST_CHILD") else {
            return;
        };
        initialize_process();
        assert_eq!(std::env::var("PA_X_PROCESS").unwrap(), "1");
        assert_eq!(std::env::var("PA_X_KEEP").unwrap(), "retained");
        for key in UPSTREAM_ENV {
            assert_eq!(std::env::var_os(key).is_some(), case == "marked", "{key}");
        }
    }

    #[test]
    fn shell_and_kernel_child_commands_strip_internal_keys() {
        let mut command = Command::new("unused");
        command.env("PRIME_AGENT_INTERNAL_NEW_ROLE", "worker");
        command.env("PA_X_KEEP", "retained");
        strip_internal_environment(&mut command);
        let keys: Vec<_> = command.get_envs().collect();
        assert!(keys
            .iter()
            .any(|(key, value)| *key == "PRIME_AGENT_INTERNAL_NEW_ROLE" && value.is_none()));
        assert!(keys
            .iter()
            .any(|(key, value)| *key == "PA_X_KEEP" && *value == Some(OsStr::new("retained"))));
    }
}
