"""fix_win7_imports.py — post-link fix for Win7-compatible builds.

Problem: newer Windows SDKs resolve CoTaskMemFree (Ole32.lib) to
``combase.dll`` (Win8+). Stock Windows 7 has no combase.dll — COM lives
in ole32.dll, which exports the identical CoTaskMemFree on every Windows
version (Win2000 -> Win11).

What this does: byte-patches the built .exe's import table entry
``combase.dll\\0`` -> ``ole32.dll\\0\\0\\0`` (same field length, null
padded). Safe because the replacement is shorter and the PE import
descriptor only needs a null-terminated name.

Usage: python3 fix_win7_imports.py <path-to-ntc.exe>
Exit 0 on success, 1 with an error message otherwise (so build scripts
can fail the build instead of shipping a binary that won't start on Win7).
"""

from __future__ import annotations

import sys


OLD = b"combase.dll\x00"
NEW = b"ole32.dll\x00\x00\x00"
assert len(OLD) == len(NEW)


def main() -> int:
    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} <path-to-exe>", file=sys.stderr)
        return 1
    path = sys.argv[1]
    with open(path, "r+b") as f:
        data = f.read()
        count = data.count(OLD)
        # Case-insensitive fallback (import names are ASCII; SDKs emit lower).
        if count == 0:
            lower = data.lower()
            count = lower.count(OLD)
            if count:
                print(
                    f"{path}: found {count} case-variant 'combase.dll' "
                    "reference(s); patching exact bytes is unsafe, aborting.",
                    file=sys.stderr,
                )
                return 1
            print(
                f"{path}: no 'combase.dll' import found; "
                "already patched or SDK emits ole32 — nothing to do.",
            )
            return 0
        patched = data.replace(OLD, NEW)
        f.seek(0)
        f.write(patched)
    print(f"{path}: patched {count} 'combase.dll' -> 'ole32.dll' import(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
