import { useEffect, useState } from 'react'
import { getSettings, setSetting } from '../api'
import { setVolumes } from '../sfx'
import BackButton from './BackButton'

const SFX_KEYS = [
  ['sfx.master', 'Master'],
  ['sfx.move', 'Move'],
  ['sfx.capture', 'Capture'],
  ['sfx.promote', 'Promote'],
  ['sfx.castle', 'Castle'],
  ['sfx.check', 'Check'],
  ['sfx.win', 'Win'],
  ['sfx.lose', 'Lose'],
] as const

const TC_OPTIONS = ['1+0', '3+2', '5+0', '10+0', '15+10', '30+0', 'unlimited']

export default function SettingsScreen({ onBack }: { onBack(): void }) {
  const [vals, setVals] = useState<Record<string, string>>({})
  const [tc, setTc] = useState('10+0')

  useEffect(() => {
    getSettings()
      .then((s) => {
        setVals(s)
        if (s['chess.tc']) setTc(s['chess.tc'])
        const vols: Record<string, number> = {}
        for (const [k] of SFX_KEYS) vols[k] = Number(s[k] ?? 80)
        setVolumes(vols)
      })
      .catch(() => {})
  }, [])

  async function change(key: string, value: string) {
    setVals((v) => ({ ...v, [key]: value }))
    try {
      await setSetting(key, value)
      if (key.startsWith('sfx.')) {
        const vols: Record<string, number> = {}
        for (const [k] of SFX_KEYS) vols[k] = Number((key === k ? value : vals[k]) ?? 80)
        setVolumes(vols)
      }
    } catch {
      /* keep local value; next load retries */
    }
  }

  return (
    <div className="screen">
      <BackButton onBack={onBack} />
      <h2>Settings</h2>
      <div className="settings">
        {SFX_KEYS.map(([k, label]) => (
          <label key={k} className="slider-row">
            <span>{label}</span>
            <input
              type="range"
              min={0}
              max={100}
              value={Number(vals[k] ?? 80)}
              onChange={(e) => change(k, e.target.value)}
            />
            <span className="slider-val">{vals[k] ?? 80}</span>
          </label>
        ))}
        <label className="slider-row">
          <span>Default clock</span>
          <select
            value={tc}
            onChange={(e) => {
              setTc(e.target.value)
              change('chess.tc', e.target.value)
            }}
          >
            {TC_OPTIONS.map((o) => (
              <option key={o} value={o}>
                {o}
              </option>
            ))}
          </select>
        </label>
        <p className="hint">Sounds play from /sounds/*.wav — drop the pack in later, no code changes.</p>
      </div>
    </div>
  )
}
