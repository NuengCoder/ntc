# Plugin architecture (2.5.0 contract)

Status: **seam shipped in 2.4.9, loading arrives in 2.5.0.**

2.4.9 ships the manifest types, the registry type and an **empty** process-wide
plugin registry. Nothing in this release reads a plugin directory, spawns a
subprocess or loads a native library. There is no user-facing `plugin` command
yet; the namespace is reserved so an alias can never shadow it.

This document is the contract 2.5.0 implements. Sections marked **(2.5.0)** are
design commitments, not code that runs today. Sections marked **(2.4.9)** describe
what is actually in the tree.

- Seam code: `src/shell/commands/plugin/{mod,manifest,registry}.rs`
- Built-in registry (single source of truth): `src/shell/commands/registry_data.rs`
- Command spec + origin: `src/shell/commands/core/spec.rs`
- Dispatch + fallback: `src/shell/commands/mod.rs`
- Refactor rationale: `big_refactor_plan.md` §7

---

## 1. Why out-of-process plugins **(2.5.0)**

Plugins are **out-of-process executables speaking JSON-RPC 2.0 over stdio**, not
native Rust dynamic libraries.

- Rust has no stable ABI; a `cdylib` plugin would break on compiler upgrades.
- A subprocess boundary works for Rust, Python, Go, Node and shell wrappers.
- A crash, hang or panic in a plugin cannot take down the shell.
- Timeouts, output limits and Ctrl-C propagation are straightforward across a
  pipe boundary.

A WASM tier may be added later for sandboxed plugins. It must reuse the same
manifest and command contract so the host integration does not fork.

---

## 2. Namespace and resolution **(2.5.0)**

Plugin commands are exposed only as:

```text
ntc plugin <plugin-id> <command> [args...]
```

- `plugin` becomes a normal built-in command module in 2.5.0 with subcommands
  `list`, `info`, `install`, `remove`, `enable`, `disable`, `run`, `trust`.
- There are **no automatic top-level command names**. This guarantees zero
  collisions with the built-ins and their aliases.
- A future opt-in `plugin-short-names` config may expose `ntc <name>` only when
  the name is not reserved. That is **not** part of 2.5.0's first cut.

Resolution order for a typed line (unchanged by the seam):

```text
alias expansion → tutorial guard → built-in registry → plugin registry → cmd_fallback
```

In 2.4.9 the plugin registry is empty and is **not consulted** by dispatch, so
`plugin …` and every other unknown token fall through to `cmd_fallback`
(`@teleport`, math, system command) exactly as before.

---

## 3. Manifest **(2.4.9 types, 2.5.0 loading)**

One TOML manifest per plugin directory. Parsed by
`PluginManifest::parse` in `src/shell/commands/plugin/manifest.rs`.

```toml
id = "com.example.hello"
name = "Hello"
version = "0.1.0"
abi = 1
entry = "hello-plugin.exe"
description = "Example plugin"

[commands.greet]
summary = { en = "Greet someone", th = "ทักทาย", zh = "...", lo = "..." }
usage = "plugin com.example.hello greet <name>"
subcommands = []
flags = ["--loud"]
```

Fields:

| Field | Required | Rule |
| --- | --- | --- |
| `id` | yes | Reverse-DNS, immutable. ≥2 dot-separated labels, each non-empty and made of lowercase ASCII letters, digits, `-` or `_`. |
| `name` | yes | Non-empty display name. |
| `version` | yes | Non-empty plugin version. |
| `abi` | yes | Host API major version. Must equal `HOST_ABI` (currently `1`). |
| `entry` | yes | Non-empty entry-point executable, relative to the plugin directory. |
| `description` | no | Defaults to empty. |
| `commands` | no | Map of bare command name → command. Defaults to empty. |

Each `[commands.<name>]`:

| Field | Required | Rule |
| --- | --- | --- |
| `summary` | yes | Locale-keyed map, at least one entry. |
| `usage` | yes | Non-empty usage line. |
| `subcommands` | no | Static subcommand names for completion. Defaults to `[]`. |
| `flags` | no | Static flag names for completion. Defaults to `[]`. |

Validation notes:

- Both the manifest and each command use `#[serde(deny_unknown_fields)]`: an
  unrecognized key is a hard error, so typos never silently disable a field.
- A command name must be non-empty, contain no whitespace and not start with `-`.
- `summary` is a `LocalizedText` map. Lookup prefers the requested locale, then
  `en`, then any available entry. Plugin strings come from this map; plugins
  never write into the compiled language packs (`src/utils/config/lang/`).
- `abi` is the host API major version. The 2.5.0 handshake returns the host's
  supported range; a manifest whose `abi` does not match is rejected at parse
  time today (`abi != HOST_ABI`).
- Static subcommands and flags are declared in the manifest so completion needs
  no round-trip. Dynamic completion over a `complete` request is a later
  milestone; the first 2.5.0 cut may ship static-only completion.

---

## 4. Registry and collision rules **(2.4.9)**

`PluginRegistry` (`src/shell/commands/plugin/registry.rs`) is an in-memory map of
manifests keyed by `id`. Registration is the only mutation; there is no loader.

`register(manifest)` rejects, in order:

1. `RegisterError::Invalid(msg)` — the manifest failed structural validation
   (§3).
2. `RegisterError::DuplicateId(id)` — a plugin with this `id` is already
   registered.
3. `RegisterError::ReservedName(name)` — the `id`, or any command name, resolves
   to a built-in name or alias via `registry_data::registry().is_reserved(name)`
   (case-insensitive, matching dispatch).

Because plugins are exposed as `plugin <id> <command>`, top-level collisions are
impossible by construction; these checks are defense in depth and also guard the
future opt-in short-name tier.

**Deferred to 2.5.0 install time (intentionally not in `register`):**

- Trust verification and the recorded manifest hash (§7).
- Run-alias collision checks (an installed plugin command must not shadow a
  user's `ral` alias). `register` only knows the built-in registry.

The process-wide accessor is `plugin::registry()` in
`src/shell/commands/plugin/mod.rs`. In 2.4.9 it returns an empty registry and is
never populated.

---

## 5. Command origin **(2.4.9)**

Every `CommandSpec` carries `source: CommandSource`. In 2.4.9 the only variant is
`CommandSource::Builtin`, and `Registry::new` asserts every spec is built-in.
2.5.0 adds a `Plugin` variant so help, completion and diagnostics can distinguish
origin without touching the 110 built-in modules.

---

## 6. Runtime contract **(2.5.0)**

Transport: JSON-RPC 2.0 over the child's stdin/stdout. stderr is reserved for the
plugin's own diagnostics and is never parsed as protocol.

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "command/run",
  "params": {
    "command": "greet",
    "args": "World",
    "cwd": "C:/work",
    "locale": "en",
    "interactive": true,
    "caps": ["fs.read:cwd"]
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "exit_code": 0,
    "stdout": "Hello, World\n",
    "effects": [{ "chdir": "C:/work/sub" }]
  }
}
```

Rules:

- **Allowed effects are explicit:** `chdir`, `set_env`, `unset_env`. A plugin
  cannot mutate `Navigator`, theme or config directly; the host applies the
  effect list after the command returns.
- **Streaming output** uses `output/chunk` notifications. The host enforces a
  timeout and a maximum buffered size.
- **Ctrl-C** sends `command/cancel`, then kills the child if it does not exit.
- **Exit code** maps to the existing `pipeline::mark_failed()` semantics, so a
  failing plugin command behaves like a failing built-in in `&&` / `||` chains.
- A handshake method (e.g. `hello`) returns the host's supported `abi` range;
  the manifest `abi` must fall inside it.

---

## 7. Security and trust **(2.5.0)**

- Plugins run with the **user's OS permissions**. This must be documented plainly
  to users; there is no privilege drop in the first cut.
- Per-project plugin directories are **not** auto-loaded. Loading requires an
  explicit `plugin trust <id>` and a recorded manifest hash.
- User-level plugins live under the ntc config directory, one directory per `id`,
  containing the manifest and entry point.
- Capabilities are declared and checked at install time. In the first 2.5.0 cut
  they scope the effect list and warn on escalation; a real sandbox is a later
  milestone (WASM tier).
- Plugin output is **untrusted text**: strip ANSI when not a TTY, and never
  evaluate it as an alias or command line.

---

## 8. Host integration: why 2.4.9 had to come first **(2.4.9)**

The refactor removed every place that hard-coded "this is the complete list of
commands". Dispatch, help, completion, alias validation and external completion
all read `registry_data::registry()`. Because the built-in registry is the single
source of truth, 2.5.0's plugin registry is **additive**: it adds a resolution
step and a `CommandSource::Plugin` variant without editing any built-in module.

Concretely, 2.5.0 will:

1. Add the `plugin` built-in command module (one new module + one line in
   `BUILTIN_SPECS`).
2. Add a loader that fills `plugin::registry()` from trusted, installed
   manifests.
3. Insert the plugin-registry lookup between the built-in registry and
   `cmd_fallback` in `execute_single_command`.
4. Extend help/completion to read plugin manifests (static subcommands/flags
   first).

None of those steps require touching the existing command modules.

---

## 9. What 2.4.9 guarantees (exit check)

**The registry can describe plugin commands but never loads one.**

Tests enforcing this:

- `plugin::manifest::tests::*` — manifest parsing, validation, locale fallback,
  rejection of bad ids / unsupported abi / empty fields / unknown keys / invalid
  command names.
- `plugin::registry::tests::cannot_shadow_a_builtin_name_or_alias` — registration
  rejects any command named like a built-in or alias (`view`, `cfg`, `god`, `r`,
  `move`).
- `plugin::registry::tests::{new_registry_is_empty, rejects_duplicate_id,
  rejects_invalid_manifest, registers_and_looks_up_a_plugin}`.
- `plugin::tests::shipped_plugin_registry_is_empty` — the process-wide registry
  is empty.
- `commands::registry_data::tests::every_spec_is_builtin` — every spec is
  `CommandSource::Builtin`; `plugin` is not a registered command.
- `commands::tests::plugin_namespace_is_reserved_but_never_loaded` — the
  namespace is reserved against alias shadowing in any case.
- `commands::tests::unknown_and_plugin_commands_reach_fallback` — neither
  `plugin` nor an unknown token resolves to a built-in, and `cmd_fallback`
  handles them.
- `commands::tests::empty_plugin_registry_leaves_help_and_completion_unchanged`
  — no help topic, no contextual root and no completions are added for the
  reserved namespace.
- `utils::config::tests::test_validate_alias_name_matches_dispatch` — alias
  validation rejects `plugin` / `PLUGIN` / `Plugin`.