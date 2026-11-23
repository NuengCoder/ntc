use crate::navigator::Navigator;
use anyhow::Result;

/// Where a command registration comes from.
///
/// 2.4.9 ships only built-ins. The `Plugin` variant arrives in 2.4.9 with
/// the out-of-process plugin registry (`commands::plugin`); carrying the
/// source on every spec now keeps that addition from touching the 110
/// built-in modules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CommandSource {
    Builtin,
}

/// Whether bare `-h` / `--help` is help for this command, or data that the
/// command must handle itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FlagPolicy {
    /// Bare `<cmd> -h/--help` prints the command's help topic.
    HelpIsHelp,
    /// `-h/--help` is data or handled inside the command.
    CommandOwnsHelp,
}

/// How a command is executed.
///
/// `Legacy` adapts the existing `fn(&str, &mut Navigator) -> Result<bool>`
/// command bodies and is the stable adapter every command uses. A `Modern`
/// variant taking a `CommandContext` is left for a future migration.
#[derive(Clone, Copy)]
pub(crate) enum Handler {
    Legacy(fn(&str, &mut Navigator) -> Result<bool>),
}

impl Handler {
    pub(crate) fn run(self, args: &str, nav: &mut Navigator) -> Result<bool> {
        match self {
            Handler::Legacy(run) => run(args, nav),
        }
    }
}

/// One command registration. Aliases live here rather than in a second list,
/// so dispatch, help and completion share the same source of truth.
#[derive(Clone, Copy)]
pub(crate) struct CommandSpec {
    /// Canonical, lowercase command name.
    pub name: &'static str,
    /// Alternate names that resolve to this command.
    pub aliases: &'static [&'static str],
    /// Help topic shown for bare `<cmd> --help`, when the command opts in.
    pub help_topic: Option<&'static str>,
    /// Whether `-h`/`--help` is help or command data.
    pub flags: FlagPolicy,
    /// Whether generic contextual help knows this command root.
    ///
    /// This preserves the previous `is_known_root` behaviour while the help
    /// renderer is migrated to registry data.
    pub contextual: bool,
    /// Aliases excluded from `contextual` when the canonical name has it.
    /// Kept for the one historical exception (`config` is contextual, `cfg`
    /// falls through to the full config topic).
    pub non_contextual_aliases: &'static [&'static str],
    /// Static subcommand/flag completions owned by this command.
    pub complete: &'static super::completion::CompletionSpec,
    /// Prompt highlighting treats the second word as a subcommand rather
    /// than a path/value. Narrower than having completions: `view` offers
    /// flags but takes a path next, while `ndb` highlights subcommands it
    /// does not complete.
    pub subcommand_highlight: bool,
    /// Whether this registration is a built-in or (2.4.9) a plugin command.
    pub source: CommandSource,
    /// The command body.
    pub run: Handler,
}
