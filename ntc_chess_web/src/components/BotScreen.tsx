import { useEffect, useState } from 'react'
import { getBots, installBot, postNew, selectBotEngine, uninstallBot } from '../api'
import { TC_OPTIONS } from '../constants'
import type { BotStatus } from '../types'
import BackButton from './BackButton'

const LEVELS = [
  ['easy', 'Easy'],
  ['normal', 'Normal'],
  ['hard', 'Hard'],
  ['master', 'Master'],
] as const
const SIDES = [
  ['white', 'White'],
  ['black', 'Black'],
  ['random', 'Random'],
] as const

function mb(bytes: number): string {
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function phaseLabel(bot: BotStatus): string {
  switch (bot.phase) {
    case 'NotInstalled':
      return 'Not installed'
    case 'Downloading':
      return 'Downloading'
    case 'Extracting':
      return 'Extracting'
    case 'Verifying':
      return 'Verifying'
    case 'Ready':
      return bot.version ? `Ready (${bot.version})` : 'Ready'
    case 'Error':
      return 'Download failed'
    default:
      return bot.phase
  }
}

export default function BotScreen({
  onBack,
  onStart,
}: {
  onBack(): void
  onStart(): void
}) {
  const [bots, setBots] = useState<BotStatus[]>([])
  const [selected, setSelected] = useState<string | null>(null)
  const [level, setLevel] = useState<'easy' | 'normal' | 'hard' | 'master'>('normal')
  const [side, setSide] = useState<'white' | 'black' | 'random'>('white')
  const [tc, setTc] = useState('10+0')
  const [notice, setNotice] = useState<string | null>(null)
  const [starting, setStarting] = useState(false)

  async function refresh() {
    try {
      const r = await getBots()
      setBots(r.bots)
      setSelected((cur) => {
        if (cur && r.bots.some((b) => b.id === cur && b.phase === 'Ready')) return cur
        const ready = r.bots.find((b) => b.phase === 'Ready')
        return ready?.id ?? null
      })
    } catch {
      /* next poll retries */
    }
  }

  useEffect(() => {
    void refresh()
    const id = window.setInterval(refresh, 900)
    return () => window.clearInterval(id)
  }, [])

  async function install(id: string) {
    setNotice(null)
    try {
      const r = await installBot(id)
      if (r.error) setNotice(r.error)
      else setBots(r.bots)
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Install failed')
    }
  }

  async function chooseEngine(id: string, engine: string) {
    setNotice(null)
    try {
      const r = await selectBotEngine(id, engine)
      if (r.error) setNotice(r.error)
      else setBots(r.bots)
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Could not switch engine')
    }
  }

  async function remove(id: string) {
    setNotice(null)
    try {
      const r = await uninstallBot(id)
      if (r.error) setNotice(r.error)
      else setBots(r.bots)
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Uninstall failed')
    }
  }

  async function start() {
    if (!selected || starting) return
    setStarting(true)
    setNotice(null)
    try {
      const ns = await postNew(tc, {
        mode: 'bot',
        bot: selected,
        level,
        side,
      })
      if (ns.error) {
        setNotice(`Could not start: ${ns.error}`)
        setStarting(false)
        return
      }
      onStart()
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Could not start game')
      setStarting(false)
    }
  }

  const ready = bots.some((b) => b.id === selected && b.phase === 'Ready')

  return (
    <div className="screen">
      <BackButton onBack={onBack} />
      <h2>Play vs AI Bot</h2>
      <p className="hint">
        Engines run locally. Install Stockfish with ntpkg (one or more versions) and pick which
        one plays below, or let ntc download it from the official release. No account and no cloud
        calls during play.
      </p>

      <div className="bot-list">
        {bots.map((bot) => {
          const pct =
            bot.total > 0 ? Math.min(100, Math.round((bot.downloaded / bot.total) * 100)) : null
          const busy =
            bot.phase === 'Downloading' ||
            bot.phase === 'Extracting' ||
            bot.phase === 'Verifying'
          return (
            <div
              key={bot.id}
              className={bot.id === selected ? 'bot-card selected' : 'bot-card'}
            >
              <div className="bot-head">
                <div>
                  <div className="bot-name">
                    {bot.name}{' '}
                    <span className="bot-license">{bot.license}</span>
                  </div>
                  <div className="hint">{bot.description}</div>
                </div>
                <label className="bot-radio">
                  <input
                    type="radio"
                    name="bot"
                    checked={bot.id === selected}
                    disabled={bot.phase !== 'Ready'}
                    onChange={() => setSelected(bot.id)}
                  />
                  <span>{phaseLabel(bot)}</span>
                </label>
              </div>

              {busy && (
                <div className="progress-wrap">
                  <div className="progress">
                    <div
                      className="progress-fill"
                      style={{ width: pct === null ? '35%' : `${pct}%` }}
                    />
                  </div>
                  <div className="hint">
                    {bot.phase === 'Downloading'
                      ? pct === null
                        ? `${mb(bot.downloaded)} downloaded`
                        : `${mb(bot.downloaded)} of ${mb(bot.total)} (${pct}%)`
                      : bot.phase === 'Extracting'
                        ? 'Extracting archive'
                        : 'Checking engine handshake'}
                  </div>
                </div>
              )}

              {bot.phase === 'Error' && bot.error && (
                <div className="bot-error">{bot.error}</div>
              )}

              {bot.engines.length > 0 && (
                <label className="bot-engine">
                  <span>Engine version</span>
                  <select
                    value={bot.engine ?? ''}
                    onChange={(e) => void chooseEngine(bot.id, e.target.value)}
                  >
                    {bot.engines.map((eng) => (
                      <option key={eng.id} value={eng.id}>
                        {eng.label}
                        {eng.source === 'ntpkg' && eng.active ? ' (ntpkg active)' : ''}
                      </option>
                    ))}
                  </select>
                </label>
              )}

              {bot.phase === 'NotInstalled' && (
                <div className="bot-hint">
                  No engine found on this machine. Install Stockfish with ntpkg and it appears here
                  automatically — for example <code>ntc ntpkg install {bot.id}</code>. The engine
                  version stays whatever you pick; it never follows <code>ntpkg use</code>.
                </div>
              )}

              <div className="btn-row">
                {bot.phase === 'NotInstalled' && (
                  <button className="primary" onClick={() => install(bot.id)}>
                    Download with ntc
                  </button>
                )}
                {bot.phase === 'Error' && (
                  <button className="primary" onClick={() => install(bot.id)}>
                    Retry download
                  </button>
                )}
                {bot.engines.some((e) => e.source === 'ntc') && (
                  <button onClick={() => remove(bot.id)}>Uninstall ntc download</button>
                )}
                {bot.homepage && (
                  <a className="bot-link" href={bot.homepage} target="_blank" rel="noreferrer">
                    Project page
                  </a>
                )}
              </div>
            </div>
          )
        })}
        {bots.length === 0 && <div className="hint">Loading bot list...</div>}
      </div>

      <h3>Game setup</h3>
      <div className="settings">
        <label className="slider-row">
          <span>Your side</span>
          <select value={side} onChange={(e) => setSide(e.target.value as typeof side)}>
            {SIDES.map(([v, label]) => (
              <option key={v} value={v}>
                {label}
              </option>
            ))}
          </select>
        </label>
        <label className="slider-row">
          <span>Difficulty</span>
          <select value={level} onChange={(e) => setLevel(e.target.value as typeof level)}>
            {LEVELS.map(([v, label]) => (
              <option key={v} value={v}>
                {label}
              </option>
            ))}
          </select>
        </label>
        <label className="slider-row">
          <span>Clock</span>
          <select value={tc} onChange={(e) => setTc(e.target.value)}>
            {TC_OPTIONS.map((o) => (
              <option key={o} value={o}>
                {o}
              </option>
            ))}
          </select>
        </label>
      </div>

      {notice && (
        <div className="notice" onClick={() => setNotice(null)}>
          {notice}
        </div>
      )}

      <div className="btn-row bot-start">
        <button className="primary big" disabled={!ready || starting} onClick={start}>
          {starting ? 'Starting...' : 'Start game'}
        </button>
        <button onClick={onBack}>Cancel</button>
      </div>
    </div>
  )
}
