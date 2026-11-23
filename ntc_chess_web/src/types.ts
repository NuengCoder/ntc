export type Square = string // "e4"

export interface LegalTarget {
  to: Square
  promo: boolean
  capture: boolean
}

export interface LastMove {
  from: Square
  to: Square
  san: string
  book?: boolean
  forced?: boolean
}

// Frozen historical position (time-travel preview, never mutated).
export interface PositionPreview {
  ply: number
  board: (string | null)[]
  turn: 'w' | 'b'
  last_move: LastMove | null
  captured_by_white: string[]
  captured_by_black: string[]
}

export interface LastResult {
  winner: 'white' | 'black' | null
  reason: string
}

export type BotPhase =
  | 'NotInstalled'
  | 'Downloading'
  | 'Extracting'
  | 'Verifying'
  | 'Ready'
  | 'Error'

export interface BotEngine {
  id: string
  label: string
  version: string | null
  source: 'ntc' | 'ntpkg' | 'system' | string
  active: boolean
}

export interface BotStatus {
  id: string
  name: string
  description: string
  version: string | null
  phase: BotPhase
  downloaded: number
  total: number
  error: string | null
  license: string
  homepage: string
  engines: BotEngine[]
  engine: string | null
}

export interface BotGameInfo {
  id: string
  name: string
  level: 'easy' | 'normal' | 'hard' | 'master' | string
  level_label: string
  color: 'w' | 'b' // the bot's side
}

export interface AiPlayerInfo {
  id: string
  name: string
  level: string
  level_label: string
  color: 'w' | 'b'
}

export interface AiGameInfo {
  white: AiPlayerInfo
  black: AiPlayerInfo
}

export interface OnlineGameInfo {
  room_id: string
  room_name: string
  color: 'w' | 'b'
  white: string | null
  black: string | null
  white_connected: boolean
  black_connected: boolean
  status: 'waiting' | 'playing' | 'finished'
  draw_offer: 'w' | 'b' | null
  rematch_offered: boolean
  coach_allowed: boolean
  coach_uses_left: number
}

export type CoachLabel =
  | 'brilliant'
  | 'best'
  | 'great'
  | 'good'
  | 'alright'
  | 'inaccuracy'
  | 'mistake'
  | 'blunder'

export interface CoachSuggestion {
  from: string
  to: string
  uci: string
  san: string
}

export interface CoachGrade {
  label: CoachLabel
  loss_cp: number
  best_cp: number
  played_cp: number
  san: string
}

export interface CoachPayload {
  enabled: boolean
  available: boolean
  ply?: number
  turn?: 'w' | 'b'
  suggestion?: CoachSuggestion | null
  best_move?: CoachSuggestion | null
  grade?: CoachGrade | null
  error?: string
  coach_uses_left?: number
}

export interface OnlineInfo {
  online: boolean
  max_rooms: number
  coach_default_limit: number
}

export interface OnlineMe {
  guest: { name: string } | null
  room: { id: string; name: string } | null
}

export interface OnlineRoomSummary {
  id: string
  name: string
  locked: boolean
  players: number
  status: 'waiting' | 'playing' | 'finished'
  host: string
  tc: string
  coach_allowed: boolean
  available_side: 'white' | 'black'
}

export interface OnlineRoomDetails extends OnlineRoomSummary {
  full: boolean
}

export interface GameState {
  board: (string | null)[] // 64 entries, index 0 = a1; "wP".."bK"
  turn: 'w' | 'b'
  status: string // ongoing|check|checkmate|stalemate|fifty-move|repetition|insufficient-material|agreement|resign|timeout|abandoned
  winner: 'white' | 'black' | null
  in_check: boolean
  last_move: LastMove | null
  moves: string[] // SAN history
  captured_by_white: string[]
  captured_by_black: string[]
  clocks: { w: number | null; b: number | null; tc: string }
  halfmove: number
  repetition: number
  hot: Square[] // hanging pieces: attackers outnumber defenders
  pinned: Square[] // pinned pieces: attacks count only vs the pinner
  silver: Square[] // relatively pinned to a pricier piece: same credit rule as pinned
  mode: 'human' | 'bot' | 'ai' | 'online'
  bot: BotGameInfo | null
  ai?: AiGameInfo | null
  online?: OnlineGameInfo | null
  last_result: LastResult | null
  // present only on move responses:
  san?: string
  sfx?: string
  error?: string
  abandoned?: boolean
  bot_move?: boolean
  bot_fallback?: boolean
  // present only on undo responses:
  undone?: LastMove
}

export interface Settings {
  [key: string]: string
}

export const TERMINAL_STATUS = new Set([
  'checkmate',
  'stalemate',
  'fifty-move',
  'repetition',
  'insufficient-material',
  'agreement',
  'resign',
  'timeout',
])
