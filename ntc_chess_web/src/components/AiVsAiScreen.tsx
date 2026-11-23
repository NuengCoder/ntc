import { useEffect, useState } from 'react'
import { getBots, postNew } from '../api'
import { TC_OPTIONS } from '../constants'
import type { BotStatus } from '../types'
import BackButton from './BackButton'

const LEVELS = [
  ['easy', 'Easy'],
  ['normal', 'Normal'],
  ['hard', 'Hard'],
  ['master', 'Master'],
] as const

type Level = (typeof LEVELS)[number][0]
type Engines = BotStatus['engines']

// Keep the pick while that build is still installed, else fall back to the
// bot's selected build, else the first one it found.
function pickEngine(list: Engines, chosen: string | null, fallback: string | null) {
  if (chosen && list.some((e) => e.id === chosen)) return chosen
  if (fallback && list.some((e) => e.id === fallback)) return fallback
  return list[0]?.id ?? null
}

export default function AiVsAiScreen({
  onBack,
  onStart,
}: {
  onBack(): void
  onStart(): void
}) {
  const [bots, setBots] = useState<BotStatus[]>([])
  const [whiteBot, setWhiteBot] = useState<string | null>(null)
  const [blackBot, setBlackBot] = useState<string | null>(null)
  const [whiteEngine, setWhiteEngine] = useState<string | null>(null)
  const [blackEngine, setBlackEngine] = useState<string | null>(null)
  const [whiteLevel, setWhiteLevel] = useState<Level>('normal')
  const [blackLevel, setBlackLevel] = useState<Level>('normal')
  const [tc, setTc] = useState('10+0')
  const [notice, setNotice] = useState<string | null>(null)
  const [starting, setStarting] = useState(false)

  // Engines can be installed/uninstalled while this screen is open, so the
  // list is refreshed until the user starts a game.
  useEffect(() => {
    let live = true
    async function refresh() {
      try {
        const r = await getBots()
        if (!live) return
        setBots(r.bots)
        const ready = r.bots.filter((b) => b.phase === 'Ready').map((b) => b.id)
        setWhiteBot((cur) => (cur && ready.includes(cur) ? cur : (ready[0] ?? null)))
        setBlackBot((cur) =>
          cur && ready.includes(cur) ? cur : (ready[ready.length - 1] ?? null),
        )
      } catch {
        /* next poll retries */
      }
    }
    void refresh()
    const id = window.setInterval(refresh, 1200)
    return () => {
      live = false
      window.clearInterval(id)
    }
  }, [])

  const ready = bots.filter((b) => b.phase === 'Ready')
  const whiteStatus = ready.find((b) => b.id === whiteBot)
  const blackStatus = ready.find((b) => b.id === blackBot)
  const whiteEngines = whiteStatus?.engines ?? []
  const blackEngines = blackStatus?.engines ?? []
  const whiteEngineId = pickEngine(whiteEngines, whiteEngine, whiteStatus?.engine ?? null)
  const blackEngineId = pickEngine(blackEngines, blackEngine, blackStatus?.engine ?? null)
  const canStart = !!whiteBot && !!blackBot && ready.length >= 1 && !starting

  async function start() {
    if (!whiteBot || !blackBot || starting) return
    setStarting(true)
    setNotice(null)
    try {
      const ns = await postNew(tc, {
        mode: 'ai',
        white: { bot: whiteBot, level: whiteLevel, engine: whiteEngineId },
        black: { bot: blackBot, level: blackLevel, engine: blackEngineId },
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

  function engineSelect(
    list: Engines,
    value: string | null,
    onPick: (id: string) => void,
  ) {
    return (
      <select value={value ?? ''} disabled={list.length === 0} onChange={(e) => onPick(e.target.value)}>
        {list.map((eng) => (
          <option key={eng.id} value={eng.id}>
            {eng.label}
            {eng.source === 'ntpkg' && eng.active ? ' (ntpkg active)' : ''}
          </option>
        ))}
        {list.length === 0 && <option value="">No engine installed</option>}
      </select>
    )
  }

  function botSelect(
    value: string | null,
    onPick: (id: string) => void,
  ) {
    return (
      <select
        value={value ?? ''}
        disabled={ready.length === 0}
        onChange={(e) => onPick(e.target.value)}
      >
        {ready.map((b) => (
          <option key={b.id} value={b.id}>
            {b.name}
            {b.engine ? '' : ' (no engine)'}
          </option>
        ))}
        {ready.length === 0 && <option value="">No engine installed</option>}
      </select>
    )
  }

  return (
    <div className="screen">
      <BackButton onBack={onBack} />
      <h2>AI vs AI</h2>
      <p className="hint">
        Two local engines play each other while you watch. Paste a link in another window to
        follow the same game; moves, times and the result are all computed by ntc. Install
        engines with <code>ntc ntpkg install stockfish</code> — anything missing shows up on the
        vs AI Bot screen.
      </p>

      <div className="settings">
        <label className="slider-row">
          <span>White engine</span>
          {botSelect(whiteBot, setWhiteBot)}
        </label>
        <label className="slider-row">
          <span>White version</span>
          {engineSelect(whiteEngines, whiteEngineId, setWhiteEngine)}
        </label>
        <label className="slider-row">
          <span>White strength</span>
          <select
            value={whiteLevel}
            onChange={(e) => setWhiteLevel(e.target.value as Level)}
          >
            {LEVELS.map(([v, label]) => (
              <option key={v} value={v}>
                {label}
              </option>
            ))}
          </select>
        </label>
        <label className="slider-row">
          <span>Black engine</span>
          {botSelect(blackBot, setBlackBot)}
        </label>
        <label className="slider-row">
          <span>Black version</span>
          {engineSelect(blackEngines, blackEngineId, setBlackEngine)}
        </label>
        <label className="slider-row">
          <span>Black strength</span>
          <select
            value={blackLevel}
            onChange={(e) => setBlackLevel(e.target.value as Level)}
          >
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
        <button className="primary big" disabled={!canStart} onClick={start}>
          {starting ? 'Starting...' : 'Start game'}
        </button>
        <button onClick={onBack}>Cancel</button>
      </div>
    </div>
  )
}
