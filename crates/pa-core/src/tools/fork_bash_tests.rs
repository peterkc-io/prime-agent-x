use std::collections::HashMap;
use std::sync::Mutex;

use super::*;

#[tokio::test]
async fn actual_bash_spawn_strips_explicit_internal_roles() {
    let directory = tempfile::tempdir().unwrap();
    let captured = Mutex::new(Vec::new());
    let on_data = |bytes: &[u8]| captured.lock().unwrap().extend_from_slice(bytes);
    let options = ExecOptions {
        on_data: &on_data,
        signal: None,
        timeout: Some(5.0),
        env: Some(HashMap::from([
            (
                "PRIME_AGENT_INTERNAL_TEST_A".to_string(),
                "poison".to_string(),
            ),
            (
                "PRIME_AGENT_INTERNAL_DAEMON_WORKER".to_string(),
                "1".to_string(),
            ),
            ("AGX_KEEP".to_string(), "retained".to_string()),
        ])),
    };
    let operations = LocalBashOperations {
        shell_path: Some("/bin/bash".to_string()),
    };
    let code = operations
        .exec("env", directory.path().to_str().unwrap(), options)
        .await
        .unwrap();
    assert_eq!(code, Some(0));
    let output = String::from_utf8(captured.into_inner().unwrap()).unwrap();
    assert!(output.contains("AGX_KEEP=retained"));
    assert!(!output
        .lines()
        .any(|line| line.starts_with("PRIME_AGENT_INTERNAL_")));
}
