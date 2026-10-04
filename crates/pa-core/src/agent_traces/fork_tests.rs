//! Both public trace entries stay disabled, even with an opt-in and credentials.
use super::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicUsize;

#[derive(Default)]
struct RecordingHttp(AtomicUsize);
impl TraceHttp for RecordingHttp {
    fn put<'a>(
        &'a self,
        _url: &'a str,
        _headers: Vec<(String, String)>,
        _body: String,
        _timeout_ms: u64,
        _cancel: Option<&'a TraceUploadCancel>,
    ) -> Pin<Box<dyn Future<Output = Result<TraceHttpResponse, TraceHttpError>> + Send + 'a>> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Err(TraceHttpError::Cancelled) })
    }
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn fork_trace_entries_never_request_even_when_requirement_is_bypassed() {
    let _env = super::tests::env_lock();
    let previous = std::env::var_os("PRIME_AGENT_TRACES_API_KEY");
    std::env::set_var("PRIME_AGENT_TRACES_API_KEY", "pa-x-test-key");
    let root = tempfile::tempdir().unwrap();
    let agent_dir = root.path().join("agent");
    let session_dir = root.path().join("sessions");
    std::fs::create_dir(&agent_dir).unwrap();
    std::fs::create_dir(&session_dir).unwrap();
    let session = session_dir.join("s.jsonl");
    std::fs::write(&session, "{\"type\":\"session\",\"id\":\"pa-x-test\",\"timestamp\":\"2026-01-01T00:00:00.000Z\",\"cwd\":\"/w\",\"version\":3}\n{\"type\":\"message\",\"id\":\"m1\",\"parentId\":null,\"message\":{\"role\":\"user\",\"content\":\"sample\",\"timestamp\":0}}\n").unwrap();
    let mut settings = crate::settings::SettingsManager::create(root.path(), &agent_dir);
    settings.set_agent_traces_enabled(true).unwrap();
    let http = RecordingHttp::default();
    let mut single = TraceUploadOptions {
        session_file: Some(&session),
        cwd: root.path(),
        agent_dir: &agent_dir,
        require_enabled: true,
        reload_config: true,
        base_url: Some("https://example.invalid"),
        http: &http,
        request_timeout_ms: 100,
        cancel: None,
        on_upload_delay: None,
    };
    // Positive control: this exact fixture reaches HTTP in the carried mechanism.
    assert!(matches!(
        super::upload::perform_agent_trace_upload_upstream(&single, None).await,
        TraceUploadResult::Failed { .. }
    ));
    assert_eq!(http.0.swap(0, Ordering::SeqCst), 1);
    for require_enabled in [true, false] {
        single.require_enabled = require_enabled;
        assert_eq!(
            super::upload_trace_file(&single).await,
            TraceUploadResult::Disabled
        );
        let all = super::upload_all_traces(&TraceUploadAllOptions {
            session_dir: Some(&session_dir),
            cwd: root.path(),
            agent_dir: &agent_dir,
            require_enabled,
            reload_config: true,
            base_url: Some("https://example.invalid"),
            http: &http,
            request_timeout_ms: 100,
            cancel: None,
            on_upload_delay: None,
            concurrency: Some(1),
            progress: None,
        })
        .await;
        assert_eq!(
            (all.total, all.uploaded, all.failed, all.skipped),
            (1, 0, 0, 1)
        );
        assert_eq!(all.results[0].1, TraceUploadResult::Disabled);
        assert_eq!(http.0.load(Ordering::SeqCst), 0);
    }
    match previous {
        Some(value) => std::env::set_var("PRIME_AGENT_TRACES_API_KEY", value),
        None => std::env::remove_var("PRIME_AGENT_TRACES_API_KEY"),
    }
}
