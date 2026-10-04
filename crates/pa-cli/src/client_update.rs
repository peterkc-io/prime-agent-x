//! The composition root's `/update` runner: the installer funnel with the
//! output captured — the TUI stays mounted while the install runs, so the
//! installer's own progress never writes to the live frame, and the
//! failure tail becomes the error row's message. It follows the same
//! update channel as `prime-agent update`. The same body
//! `prime-agent update` runs (pa-core's installer module), so the TUI and
//! the CLI cannot diverge.

use pa_core::update::installer::{self, InstallerOutput};

/// The `/update` funnel handle (the interactive options carry it).
#[derive(Clone, Default)]
pub struct ClientUpdate;

impl pa_tui::update_command::UpdateCommands for ClientUpdate {
    fn unavailable_reason(&self) -> Option<&'static str> {
        Some(pa_types::fork_identity::UPDATE_INSTRUCTIONS)
    }
    fn run_update(&self) -> pa_tui::update_command::UpdateRunFuture {
        Box::pin(async {
            if pa_types::fork_identity::UPDATES_DISABLED {
                return Err(pa_types::fork_identity::UPDATE_INSTRUCTIONS.to_string());
            }
            match installer::run_installer(
                Some(crate::installer_update::requested_installer_channel(None)),
                InstallerOutput::Capture,
            )
            .await
            {
                Ok(installed) => Ok(installed
                    .version
                    .unwrap_or_else(|| "the latest build".to_string())),
                Err(failure) => Err(failure.message),
            }
        })
    }
}
