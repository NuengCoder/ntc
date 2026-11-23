import type {
  BotStatus,
  CoachPayload,
  GameState,
  LegalTarget,
  OnlineInfo,
  OnlineMe,
  OnlineRoomDetails,
  OnlineRoomSummary,
  PositionPreview,
  Settings,
} from './types'

/** Error carrying the server's machine code (`room_full`, `bad_password`, …). */
export class ApiError extends Error {
  status: number
  code: string

  constructor(status: number, code: string, message: string) {
    super(message)
    this.name = 'ApiError'
    this.status = status
    this.code = code
  }
}

async function api<T>(url: string, init?: RequestInit): Promise<T> {
  const res = await fetch(url, init)
  if (!res.ok) {
    const text = await res.text()
    let code = `http_${res.status}`
    let msg = text
    try {
      const j = JSON.parse(text)
      code = j.error || code
      msg = j.message || j.error || text
    } catch {
      /* plain body */
    }
    throw new ApiError(res.status, code, msg || `HTTP ${res.status}`)
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

export interface AiPlayerOption {
  bot: string
  level: 'easy' | 'normal' | 'hard' | 'master'
  engine?: string | null
}

export interface NewGameOptions {
  mode?: 'human' | 'bot' | 'ai'
  bot?: string
  level?: 'easy' | 'normal' | 'hard' | 'master'
  side?: 'white' | 'black' | 'random'
  white?: AiPlayerOption
  black?: AiPlayerOption
}

export function postNew(tc: string, opts?: NewGameOptions): Promise<GameState> {
  return api('/api/chess/new', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ tc, ...opts }),
  })
}

export function getBots(): Promise<{ bots: BotStatus[] }> {
  return api('/api/chess/bots')
}

export function installBot(id: string): Promise<{ ok?: boolean; bots: BotStatus[]; error?: string }> {
  return api('/api/chess/bots/install', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ id }),
  })
}

export function uninstallBot(
  id: string,
): Promise<{ ok?: boolean; bots: BotStatus[]; error?: string }> {
  return api('/api/chess/bots/uninstall', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ id }),
  })
}

export function selectBotEngine(
  id: string,
  engine: string,
): Promise<{ ok?: boolean; bots: BotStatus[]; error?: string }> {
  return api('/api/chess/bots/select', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ id, engine }),
  })
}

export function postBotMove(ply: number): Promise<GameState> {
  return api('/api/chess/bot-move', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ ply }),
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

export function postCoach(): Promise<CoachPayload> {
  return api('/api/chess/coach', { method: 'POST', headers: json, body: '{}' })
}

// ── Online (available when the host ran `serve chess --online`) ──────────

export function serverHost(): string {
  return window.location.host || window.location.hostname || 'localhost'
}

export function getOnlineInfo(): Promise<OnlineInfo> {
  return api('/api/online/info')
}

export function getOnlineMe(): Promise<OnlineMe> {
  return api('/api/online/me')
}

export function onlineLogin(name: string): Promise<{ guest: { name: string } }> {
  return api('/api/online/login', {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ name }),
  })
}

export function onlineLogout(): Promise<{ ok: boolean }> {
  return api('/api/online/logout', { method: 'POST', headers: json, body: '{}' })
}

export function listOnlineRooms(q: string): Promise<{ rooms: OnlineRoomSummary[]; count: number }> {
  return api(`/api/online/rooms?q=${encodeURIComponent(q)}`)
}

export function getOnlineRoom(id: string): Promise<OnlineRoomDetails> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}`)
}

export interface CreateRoomInput {
  name: string
  password?: string
  tc: string
  side: 'white' | 'black' | 'random'
  coach_allowed: boolean
}

export function createOnlineRoom(
  input: CreateRoomInput,
): Promise<{ room: OnlineRoomSummary }> {
  return api('/api/online/rooms', {
    method: 'POST',
    headers: json,
    body: JSON.stringify(input),
  })
}

export function joinOnlineRoom(
  id: string,
  password?: string,
): Promise<{ room: OnlineRoomSummary }> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/join`, {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ password: password ?? '' }),
  })
}

export function quickJoinOnline(): Promise<{ room: OnlineRoomSummary }> {
  return api('/api/online/quick-join', { method: 'POST', headers: json, body: '{}' })
}

export function getOnlineState(id: string, signal?: AbortSignal): Promise<GameState> {
  return api(
    `/api/online/rooms/${encodeURIComponent(id)}/state`,
    signal ? { signal } : undefined,
  )
}

export function getOnlineMoves(id: string, sq: string): Promise<{ moves: LegalTarget[] }> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/moves?sq=${encodeURIComponent(sq)}`)
}

export function getOnlinePosition(id: string, ply: number): Promise<PositionPreview> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/position?ply=${ply}`)
}

export function onlineMove(
  id: string,
  from: string,
  to: string,
  promo?: string,
): Promise<GameState> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/move`, {
    method: 'POST',
    headers: json,
    body: JSON.stringify(promo ? { from, to, promo } : { from, to }),
  })
}

export function onlineResign(id: string): Promise<GameState> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/resign`, {
    method: 'POST',
    headers: json,
    body: '{}',
  })
}

export function onlineDraw(
  id: string,
  action: 'offer' | 'accept',
): Promise<GameState> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/draw`, {
    method: 'POST',
    headers: json,
    body: JSON.stringify({ action }),
  })
}

export function onlineRematch(id: string): Promise<GameState> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/rematch`, {
    method: 'POST',
    headers: json,
    body: '{}',
  })
}

export function onlineCoach(
  id: string,
  signal?: AbortSignal,
): Promise<{ coach: CoachPayload }> {
  return api(
    `/api/online/rooms/${encodeURIComponent(id)}/coach`,
    signal ? { method: 'POST', body: '{}', signal } : { method: 'POST', body: '{}' },
  )
}

export function leaveOnlineRoom(id: string): Promise<{ ok: boolean }> {
  return api(`/api/online/rooms/${encodeURIComponent(id)}/leave`, {
    method: 'POST',
    headers: json,
    body: '{}',
  })
}
