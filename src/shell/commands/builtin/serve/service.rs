//! The single `serve` service table.
//!
//! Everything that needs to know which services exist — the `serve`
//! dispatcher, `serve` help, `showcg`, `config serve` and completion —
//! reads this table. Adding a service means adding one row here plus its
//! module, its lang keys and its `ServeService` variant.

use super::{arc, backup, chess, config, fake, files, git, http_server, mdview, ndb, theme};
use crate::config::ServeService;
use crate::navigator::Navigator;
use anyhow::Result;

/// One `serve <id>` service.
pub(crate) struct ServeServiceSpec {
    /// The token the user types after `serve`.
    pub id: &'static str,
    /// The configuration slot for this service's port/bind defaults.
    pub service: ServeService,
    /// `serve` help row (`cmd.serve.services_row_*`).
    pub row_key: &'static str,
    /// `showcg` label (`msg.showcg.label_serve_*`).
    pub label_key: &'static str,
    pub run: fn(&str, &mut Navigator) -> Result<bool>,
}

pub(crate) static SERVICES: &[ServeServiceSpec] = &[
    ServeServiceSpec {
        id: "files",
        service: ServeService::Files,
        row_key: "cmd.serve.services_row_files",
        label_key: "msg.showcg.label_serve_files",
        run: files::cmd_serve_files,
    },
    ServeServiceSpec {
        id: "http-server",
        service: ServeService::Http,
        row_key: "cmd.serve.services_row_http",
        label_key: "msg.showcg.label_serve_http",
        run: http_server::cmd_serve_http,
    },
    ServeServiceSpec {
        id: "chess",
        service: ServeService::Chess,
        row_key: "cmd.serve.services_row_chess",
        label_key: "msg.showcg.label_serve_chess",
        run: chess::cmd_serve_chess,
    },
    ServeServiceSpec {
        id: "git",
        service: ServeService::Git,
        row_key: "cmd.serve.services_row_git",
        label_key: "msg.showcg.label_serve_git",
        run: git::cmd_serve_git,
    },
    ServeServiceSpec {
        id: "arc",
        service: ServeService::Arc,
        row_key: "cmd.serve.services_row_arc",
        label_key: "msg.showcg.label_serve_arc",
        run: arc::cmd_serve_arc,
    },
    ServeServiceSpec {
        id: "theme",
        service: ServeService::Theme,
        row_key: "cmd.serve.services_row_theme",
        label_key: "msg.showcg.label_serve_theme",
        run: theme::cmd_serve_theme,
    },
    ServeServiceSpec {
        id: "ndb",
        service: ServeService::Ndb,
        row_key: "cmd.serve.services_row_ndb",
        label_key: "msg.showcg.label_serve_ndb",
        run: ndb::cmd_serve_ndb,
    },
    ServeServiceSpec {
        id: "backup",
        service: ServeService::Backup,
        row_key: "cmd.serve.services_row_backup",
        label_key: "msg.showcg.label_serve_backup",
        run: backup::cmd_serve_backup,
    },
    ServeServiceSpec {
        id: "fake",
        service: ServeService::Fake,
        row_key: "cmd.serve.services_row_fake",
        label_key: "msg.showcg.label_serve_fake",
        run: fake::cmd_serve_fake,
    },
    ServeServiceSpec {
        id: "mdview",
        service: ServeService::Mdview,
        row_key: "cmd.serve.services_row_mdview",
        label_key: "msg.showcg.label_serve_mdview",
        run: mdview::cmd_serve_mdview,
    },
    ServeServiceSpec {
        id: "config",
        service: ServeService::Config,
        row_key: "cmd.serve.services_row_config",
        label_key: "msg.showcg.label_serve_config",
        run: config::cmd_serve_config,
    },
];

/// Look up a service by its dispatch token.
pub(crate) fn find(id: &str) -> Option<&'static ServeServiceSpec> {
    SERVICES.iter().find(|s| s.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn service_ids_are_unique() {
        let ids: HashSet<&str> = SERVICES.iter().map(|s| s.id).collect();
        assert_eq!(ids.len(), SERVICES.len());
    }

    /// Every `ServeService` variant must have exactly one row, so `showcg`
    /// and `config serve` can never miss a service.
    #[test]
    fn every_serve_service_variant_has_a_row() {
        let all = [
            ServeService::Files,
            ServeService::Http,
            ServeService::Chess,
            ServeService::Git,
            ServeService::Arc,
            ServeService::Theme,
            ServeService::Ndb,
            ServeService::Backup,
            ServeService::Fake,
            ServeService::Config,
            ServeService::Mdview,
        ];
        for svc in all {
            assert!(
                SERVICES.iter().any(|s| s.service == svc),
                "missing serve row for {}",
                svc.name()
            );
        }
        assert_eq!(SERVICES.len(), all.len());
    }

    #[test]
    fn dispatch_ids_round_trip_through_find() {
        for spec in SERVICES {
            assert_eq!(find(spec.id).map(|s| s.id), Some(spec.id));
        }
        assert!(find("nope").is_none());
    }

    /// The `serve` completion root must offer exactly the dispatchable
    /// service ids (plus `help` and flags), so tab-completion can never
    /// drift from the single service table.
    #[test]
    fn completion_root_service_ids_match_the_table() {
        let table_ids: HashSet<&str> = SERVICES.iter().map(|s| s.id).collect();
        let completion_ids: HashSet<&str> =
            crate::shell::commands::builtin::serve::completion::COMPLETION
                .root
                .iter()
                .copied()
                .filter(|t| !t.starts_with('-') && *t != "help")
                .collect();
        assert_eq!(
            completion_ids, table_ids,
            "serve completion root and SERVICES table disagree"
        );
    }
}
