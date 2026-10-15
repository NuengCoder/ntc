import type { GameState, LegalTarget, PositionPreview, Settings } from './types'

async function api<T>(url: string, init?: RequestInit): Promise<T> {
  const res = await fetch(url, init)
  if (!res.ok) {
    const text = await res.text()
    let msg = text
    try {
      const j = JSON.parse(text)
      msg = j.error || text
    } catch {
      /* plain body */
    }
    throw new Error(msg || `HTTP ${res.status}`)
  }
  return res.json()
}

const json = { 'Content-Type': 'application/json' }

export function getState(signal?: AbortSignal): Promise<GameState> {
  return api('/api/chess/state', signal ? { signal } : undefined)
}

export function getMoves(sq: string): Promise<{ moves: LegalTarget[] }> {
  return api(`/api/chess/moves?sq=${encodeURIComponent(sq)}`)
}

export function postMove(from: string, to: string, promo?: string): Promise<GameState> {
  return api('/api/chess/move', {
    method: 'POST',
    headers: json,
    body: JSON.stringify(promo ? { from, to, promo } : { from, to }),
  })
}

export function postNew(tc: string): Promise<GameState> {
  return api('/api/chess/new', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ tc }),
  })
}

export function postResign(color: 'white' | 'black'): Promise<GameState> {
  return api('/api/chess/resign', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ color }),
  })
}

export function postDraw(): Promise<GameState> {
  return api('/api/chess/draw', { method: 'POST', headers: json, body: '{}' })
}

export function postRematch(): Promise<GameState> {
  return api('/api/chess/rematch', { method: 'POST', headers: json, body: '{}' })
}

export function postUndo(): Promise<GameState> {
  return api('/api/chess/undo', { method: 'POST', headers: json, body: '{}' })
}

export function getPosition(ply: number): Promise<PositionPreview> {
  return api(`/api/chess/position?ply=${ply}`)
}

export function getSettings(): Promise<Settings> {
  return api('/api/chess/settings')
}

export function setSetting(key: string, value: string): Promise<{ ok: boolean }> {
  return api('/api/chess/settings', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ key, value }),
  })
}
