# pa-x: a separate Prime Agent fork

pa-x keeps upstream settings and session formats while separating its executable,
daemon namespace, and default kernel venv.

```text
upstream prime-agent ---- upstream sockets / kernel-venv
         |                              |
         +------ shared settings -------+
         +------ shared sessions -------+
         |                              |
pa-x ------------------- pa-x sockets / kernel-venv-pa-x
         |
         +--- updates, telemetry, and trace uploads disabled
         +--- settings overlay and plugins planned, not implemented
```

## Current foundation

The comparison uses upstream source defaults at `c24ac227f`. An installed
upstream launcher can override its source defaults.

| Feature | Upstream source | pa-x foundation |
| --- | --- | --- |
| Executable and version name | `prime-agent` | `pa-x` |
| Unix daemon directory | `prime-agent-<suffix>` | `pa-x-<suffix>` |
| Windows pipe prefix | `prime-agent-` | `pa-x-` |
| Default kernel venv | `kernel-venv` | `kernel-venv-pa-x` |
| Agent and session stores | Existing formats and overrides | Same formats and overrides |
| Inherited upstream role variables | No fork startup reset | Cleared in fresh processes |
| Upstream socket use | No fork refusal | Rejected before connect, bind, or stale cleanup |
| Updates | Upstream update path | Disabled with local install instructions |
| Usage telemetry | Settings and environment control | Release switch forced off; debug behavior retained for tests |
| Trace uploads | Saved opt-in and credentials | Disabled, including forced upload options |
| Settings overlay | Not assessed in this comparison | Planned, not implemented |
| Plugins and plugin workflows | Not assessed in this comparison | Planned, not implemented |

The runtime policy has focused tests for startup reset, marked child behavior,
socket resolution, telemetry, and public single/all trace uploads. The public
trace rejection test includes a reachable HTTP positive control. Carried
upstream protocol tests still cover the private upload mechanism; they do not
enable the fork's public path.

Release builds never send usage telemetry or write `telemetry.jsonl`, even with
saved and environment opt-ins. Debug builds retain upstream counting and the
local mirror so the upstream tests stay unchanged. Use an isolated HOME for debug
runs. No `telemetry-agx.jsonl` file is created.

The upstream client factory still creates `telemetry.json` for its installation
id, even when the release switch is off. This is a local write in the shared agent
dir. The fork does not change installation-id handling.

The native packer builds the local bundle. No upstream installation is replaced.
[INSTALL.md](INSTALL.md) gives the commands and shared-state warning.

## Compatibility boundaries

- Settings and sessions remain shared in this foundation. An edit can affect the
  upstream program. Do not assume the planned overlay already exists.
- `PA_X_PROCESS` marks fork descendants. It does not bypass socket refusal.
- Kernel and bash-tool children must not inherit internal daemon role variables.
- Telemetry and trace opt-ins remain readable in shared settings, but cannot
  enable the fork's production upload paths.
- Root `README.md` remains upstream material. This document describes the fork.
- Upstream release workflows are not a pa-x release process.

## Maintaining the fork

[SEAMS.md](SEAMS.md) lists every changed upstream file. Run:

```sh
python3 scripts/pa-x/check_seams.py
python3 scripts/pa-x/test_check_seams.py
```

The checker compares with `upstream/main`. It rejects both an unlisted changed
upstream file and a row for an unchanged file. Fork-owned new files do not need
an upstream seam row. Update the ledger with each later upstream modification.

## Planned work

Settings overlays and plugins are later approved-plan actions. Neither feature
is delivered by this foundation. They must preserve the runtime and privacy
boundaries above.
