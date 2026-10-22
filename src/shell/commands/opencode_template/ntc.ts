import { tool } from "@opencode-ai/plugin"

/**
 * ntc — Navigate, Toolkit, Center.
 *
 * One generic tool that runs any ntc command in the project and returns its
 * output. ntc supports one-shot execution (`ntc <command> [args...]`), which
 * is exactly what this tool wraps.
 */

const DEFAULT_TIMEOUT_MS = 120_000

/** Strip ANSI/VT escape sequences so command output is clean for the model. */
function stripAnsi(input: string): string {
  // eslint-disable-next-line no-control-regex
  return input.replace(/\u001B\[[0-9;?]*[ -/]*[@-~]/g, "")
}

export default tool({
  description: [
    "Run an ntc command and return its output.",
    'ntc ("Navigate, Toolkit, Center") is a terminal toolkit. Pass the command',
    'exactly as you would type it in the ntc shell, e.g. "gs TODO", "report md src",',
    '"scan dir .", "git status" or "ran test". ntc runs one command and exits.',
    "",
    "Command families:",
    "- nav: cd, back (b), where, gos, gosc, godrive (god), fgo",
    "- find: gs, fs, ds, locate, fsc, qur, scan",
    "- inspect: view, report, txtc, txtf, size",
    "- tasks: ran, scr (script), ral, run (r)",
    "- config: config, local, init, deinit, ignore, care, igcare",
    "- git: git status|branch|log|diff|add|commit|push|...",
    "- data: ndb (SQLite), nsheet (CSV/TSV/Excel)",
    "- net: http, dl, up, ping, dns, smtp, serve, nssh, ipcfg",
    "- misc: qr, clip, math, encode, decode, fake, todo (td), arc, tmprt, theme, mode, ntpkg",
    "",
    "Avoid interactive/TUI commands (ne, nfe, sys, ui, nms, gos, gosc, serve,",
    "bare ndb/nsheet) — they require a real terminal and will time out.",
  ].join("\n"),
  args: {
    command: tool.schema
      .string()
      .describe('The ntc command line to run, e.g. "gs TODO" or "report md src"'),
    cwd: tool.schema
      .string()
      .optional()
      .describe("Working directory (defaults to the session directory)"),
    timeout: tool.schema
      .number()
      .optional()
      .describe("Timeout in milliseconds (default 120000)"),
  },
  async execute(args, context) {
    const command = args.command.trim()
    if (!command) {
      return 'No command given. Example: use the ntc tool with command "gs TODO".'
    }

    const cwd = args.cwd ?? context.directory
    const timeout = args.timeout ?? DEFAULT_TIMEOUT_MS

    const binary = Bun.which("ntc")
    if (!binary) {
      return [
        "Could not find the `ntc` binary on PATH.",
        "Install it with `npm install -g @nuengcoder/ntc`, or add it to PATH, then retry.",
      ].join("\n")
    }

    const env = {
      ...process.env,
      NO_COLOR: "1",
      CLICOLOR: "0",
      CLICOLOR_FORCE: "0",
      FORCE_COLOR: "0",
    }

    // npm installs on Windows expose ntc.cmd/.bat shims that need a shell.
    const needsShell = /\.(cmd|bat)$/i.test(binary)
    const argv = needsShell
      ? ["cmd", "/d", "/s", "/c", binary, command]
      : [binary, command]

    let proc
    try {
      proc = Bun.spawn(argv, {
        cwd,
        env,
        stdin: "ignore",
        stdout: "pipe",
        stderr: "pipe",
      })
    } catch (error) {
      return `Failed to start ntc in ${cwd}: ${String(error)}`
    }

    let timedOut = false
    const timer = setTimeout(() => {
      timedOut = true
      proc.kill()
    }, timeout)

    const [stdout, stderr] = await Promise.all([
      new Response(proc.stdout).text(),
      new Response(proc.stderr).text(),
    ])
    const exitCode = await proc.exited
    clearTimeout(timer)

    const out = stripAnsi(stdout).trim()
    const err = stripAnsi(stderr).trim()

    const sections: string[] = []
    if (out) sections.push(out)
    if (err) sections.push(`stderr:\n${err}`)
    if (timedOut) {
      sections.push(
        `ntc did not finish within ${timeout}ms. This usually means the command is interactive (a TUI); run it in a real terminal instead.`,
      )
    } else if (exitCode !== 0) {
      sections.push(`ntc exited with code ${exitCode}.`)
    }

    return sections.join("\n\n") || "(ntc produced no output)"
  },
})
