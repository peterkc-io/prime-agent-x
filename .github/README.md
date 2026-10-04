# pa-x

A Prime Agent fork with a separate executable and runtime namespace.

## Why fork

- [Separate daemon and kernel namespaces](../docs/fork/FORK.md) let pa-x run beside upstream.
- [Startup environment reset](../docs/fork/FORK.md) prevents inherited upstream roles from selecting a fork process mode.
- [Updates, telemetry, and trace uploads are disabled](../docs/fork/FORK.md) in the foundation.
- [A local native bundle installer](../docs/fork/INSTALL.md) leaves the upstream installation intact.
- [Settings overlays and plugins are planned](../docs/fork/FORK.md), not implemented.

## Start here

Read [the fork comparison and compatibility boundaries](../docs/fork/FORK.md).
Use [the local installation instructions](../docs/fork/INSTALL.md), not the
upstream installer, for `pa-x`.

Settings and sessions are shared in this foundation. Root `README.md` remains
upstream material. [The seam ledger](../docs/fork/SEAMS.md) records changes to
upstream files.

## Acknowledgements

This fork builds on Prime Intellect's upstream Prime Agent and its use of
ratatui and vouch. The extension design uses **patterns and ideas adapted from**
[OpenAI Codex](https://github.com/openai/codex), licensed under Apache-2.0.
This statement credits design sources, not copied Codex code.
