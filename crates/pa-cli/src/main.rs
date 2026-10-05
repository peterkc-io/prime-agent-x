fn main() {
    pa_types::fork_identity::initialize_process();
    if std::env::var_os("PRIME_AGENT_KERNEL_VENV").is_none_or(|value| value.is_empty()) {
        let home = pa_types::platform::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));
        std::env::set_var(
            "PRIME_AGENT_KERNEL_VENV",
            home.join(".prime").join("agent").join("kernel-venv-pa-x"),
        );
    }
    // Allocator tuning before any thread spawns: the session-load and
    // attach-snapshot phases are large transient bursts, and glibc's
    // per-thread arenas otherwise keep each burst's high-water pages
    // resident for the process lifetime.
    pa_types::memory_release::cap_thread_arenas();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = pa_cli::main_with_runtime(&args, &pa_cli::PrintRuntime);
    std::process::exit(code);
}
