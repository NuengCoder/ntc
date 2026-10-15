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
  last_result: LastResult | null
  // present only on move responses:
  san?: string
  sfx?: string
  error?: string
  abandoned?: boolean
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
