# NSSH: read pass-env from deadlock vault (silent master cache) + rmf vault protection + OS-level note

## Goal

1. **`nssh ssh <profile>` with `pass_env`:** read the pass-env secret from the
   deadlock vault **silently** (no master-password prompt) and use it for SSH
   auth. On SSH auth failure, prompt the user (hidden) for their own password
   and retry. On continued failure, return to the ntc shell.

2. **`rmf deadlock.vault`:** block removal of the vault file via ntc's `rmf`
   unless running as root (Unix) or Administrator (Windows).

## Problem analysis

- The `deadlock` shell **forces** the user to change the master password on
  first run (deadlock/mod.rs:40-43). So `DEFAULT_MASTER` ("12345678") no longer
  unlocks the vault → nssh's silent `DEFAULT_MASTER` unlock always fails → the
  pass-env secret is never read.
- `rmf deadlock.vault` (from any CWD) resolved to a different path than
  `vault_path()`, so the comparison failed and `rmf` deleted the vault.

## Decisions

1. **Session master cache:** add a process-global `Mutex<Option<String>>` in
   `crate::deadlock` holding the master password. `run_deadlock_shell()`
   caches it after successful unlock; clears it on exit (including early/error
   exits via `Drop`). nssh reads it silently — no prompt.
2. **`build_auth` vault order:** try cached master → `DEFAULT_MASTER` → env var
   fallback. No interactive master prompt ever in nssh.
3. **Auth retry:** on `is_auth_failure`, prompt (hidden) for the user's own
   password up to `MAX_AUTH_ATTEMPTS = 3`; empty input cancels → back to shell.
4. **rmf vault protection:** `is_deadlock_vault(path)` uses canonicalized path
   comparison; blocks non-root/non-admin removal in both direct `rmf` and
   wildcard `remove_rmf_pattern`.
5. **OS-level removal** (File Explorer / `rm -rf`): **not preventable by ntc code**
   — the user is the file owner. The vault already has 0600 / owner-only ACL at
   the OS level. Document `chattr +i /Users/.../deadlock.vault` on Linux as an
   optional hardening step. This is out of scope for code changes.

## Task list

### 1. `src/deadlock/mod.rs` — vault master cache

- Add a global cache:
  ```rust
  static MASTER_CACHE: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
  ```
- Public functions:
  - `pub fn cache_master(master: &str)` — store.
  - `pub fn clear_master_cache()` — clear.
  - `pub fn get_cached_master() -> Option<String>` — clone out.
- Modify `run_deadlock_shell()`:
  - After `unlock(&path)` succeeds, before `shell_loop`, call
    `cache_master(&vault.master())`.
  - Wrap the `shell_loop` call so `clear_master_cache()` runs on all exit paths
    (use a guard struct with `Drop`, or explicit after `shell_loop` returns +
    `Ok(...)?` error paths).
- Add `pub fn master(&self) -> &str { &self.master }` to `Vault` in vault.rs
  (deadlock/vault.rs).

### 2. `src/deadlock/vault.rs` — expose master

- Add `impl Vault { pub fn master(&self) -> &str }` (returns `&self.master`).

### 3. `src/shell/commands/nssh.rs` — silent vault unlock in build_auth

- Rewrite `build_auth(profile)`:
  - If `profile.pass_env` is set:
    a. Try `crate::deadlock::get_cached_master()` → `Vault::unlock(path, cached)` →
       `vault.get(var)`. If found → use it.
    b. Try `Vault::unlock(path, DEFAULT_MASTER)` → `vault.get(var)`. If found → use it.
    c. Environment variable fallback.
  - Return `SshAuth { key_path, password }`.
- `cmd_ssh` retry loop (already implemented): on `is_auth_failure`, prompt hidden
  for own password up to `MAX_AUTH_ATTEMPTS (3)`, then back to shell.
- Bare `nssh` delegates to `cmd_ssh`.
- `cmd_cp` uses `build_auth(&profile)`.

### 4. `src/shell/commands/misc.rs` — rmf vault protection

- `is_root_or_admin()`, `is_deadlock_vault(path)` (canonicalized comparison) —
  already added in the prior code session.
- `cmd_rmf` blocks vault removal for non-root/admin (both direct and wildcard paths).

### 5. Validation

- `cargo check --all-features`, `cargo clippy --all-targets --all-features`, `cargo test --all-features`.
- Manual (requires a `deadlock` vault):
  1. `deadlock` → `add rocky-pass secret_password` → set a **custom** master → `exit`.
  2. `nssh add rocky --host X --user U --pass-env rocky-pass` (env var unset).
  3. `nssh ssh rocky` → reads `rocky-pass` from the cached vault **without** prompting
     for the master password; attempts SSH. Wrong password → reprompts user's own
     password up to 3 attempts → returns to ntc shell.
  4. Fresh ntc process (no prior `deadlock` shell) → vault has custom master →
     nssh falls back to env var (no master prompt).
  5. `rmf deadlock.vault` (from any CWD, non-admin) → refused.
  6. `rmf deadlock.vault` as root/admin → allowed.

## Out of scope

- OS-level deletion (File Explorer, `rm -rf`): the OS already restricts the file
  to the owner (0600/ACL); true immutability requires `chattr +i` (Linux). Not an
  ntc code change.
- Cross-process vault caching (separate ntc process): the master cache is
  process-local; a second terminal window will fall back to `DEFAULT_MASTER` or
  env var (documented limitation).
