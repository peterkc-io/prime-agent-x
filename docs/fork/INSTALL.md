# Install and update pa-x locally

Build a verified checkout and install a separate `pa-x` bundle. Automatic
updates are disabled.

## Layout and limits

```text
verified checkout -> native release packer -> ~/.local/share/pa-x/
                                          -> ~/.local/bin/pa-x

pa-x daemon       -> <TMPDIR>/pa-x-<suffix>/
pa-x kernel       -> ~/.prime/agent/kernel-venv-pa-x/
settings/sessions -> ~/.prime/agent/ (shared with upstream)
```

The local installer supports Unix hosts. Windows runtime pipe names use the
`pa-x-` prefix, but this installer does not install Windows binaries.

**Settings and sessions are still shared.** The foundation does not provide a
settings overlay. `PRIME_AGENT_CODING_AGENT_DIR` and the existing session-dir
overrides remain supported.

The installer writes only the `pa-x` launcher and bundle. It does not replace
`prime-agent`, its installation, or its kernel venv. Bundled runtime assets can
retain upstream names inside the `pa-x` bundle.

## Install

Requirements: Rust 1.98.1, Python 3, and the repository's native build tools.
The default command downloads the model catalog. Cargo can download locked
crate dependencies. No release credentials are required.

From the verified checkout:

```sh
scripts/pa-x/install-local.sh
~/.local/bin/pa-x --version
```

The version output starts with `pa-x `. The package contains the native runtime,
skills, and generated catalog assets used by the upstream release packer.
The first real kernel start uses the separate `kernel-venv-pa-x` venv.

For an already built release binary and a generated catalog directory:

```sh
scripts/pa-x/install-local.sh --skip-build --catalog-assets <catalog-directory>
```

To test installation without changing the real installation, use a fresh,
resolved temporary `HOME`. Do not point it at the real home through a symlink.
The installer rejects symlinked install directories and restores the previous
bundle if launcher replacement fails.

## Update

Run the installer again from a newly verified checkout. It stages the new
bundle before replacing the previous bundle and launcher.

`pa-x update`, `pa-x update --nightly`, `pa-x u`, and TUI `/update` exit with an
error and these local installation instructions. They do not download an
upstream installer or query a release endpoint.

## Runtime isolation

A fresh `pa-x` process clears inherited upstream socket, kernel-venv, kernel
owner, package-dir, and `PRIME_AGENT_INTERNAL_*` variables. It sets
`PA_X_PROCESS=1`. Marked fork children keep their required inherited values.
Bash-tool and kernel children remove internal role variables before spawn.

Do not set `PA_X_PROCESS` as a general shell setting. It is a process-lineage
marker, not permission to use upstream sockets. Flag, environment, and default
socket paths reject the upstream `prime-agent-<uid or user>` and
`prime-agent-rust-<numeric uid>` directories before an operation opens them.
Existing symlink parents are resolved for this check.

Release usage telemetry and all public trace uploads stay off even if shared
settings opt in. Debug builds retain upstream usage telemetry and its local
mirror for tests; use an isolated HOME. The release client still creates the
shared `telemetry.json` installation-id file, but never writes `telemetry.jsonl`.

The executable sets its default venv through `PRIME_AGENT_KERNEL_VENV` before
command dispatch. If that default parent is unwritable, bootstrap fails rather
than using upstream's XDG fallback. Marked children can retain an explicit venv.
See [FORK.md](FORK.md) for current and planned features, and
[SEAMS.md](SEAMS.md) for the upstream change inventory.
