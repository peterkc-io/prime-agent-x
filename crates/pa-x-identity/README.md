# pa-x-identity

Fork identity and process-boundary policy for `pa-x`.

## Scope

This leaf crate owns startup environment isolation, internal-role removal for
child commands, upstream socket refusal, and the disabled-update error.
`pa-types` re-exports its API as `fork_identity` for existing crate boundaries.
It has no runtime dependencies and does not depend on another workspace crate.

## Non-goals

It does not own settings, sessions, installers, transport I/O, or plugins.

## Public API

`initialize_process` marks and isolates a fork process. `strip_internal_environment`
removes internal roles from child commands. `reject_upstream_socket` rejects
upstream endpoints before transport I/O. Policy constants and `UpdateUnavailable`
keep fork update and upload decisions separate from upstream implementation.
