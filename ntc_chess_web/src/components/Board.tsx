import { memo, useEffect, useRef, useState } from 'react'
import { theme } from '../theme'
import type { LegalTarget } from '../types'

export interface PieceGlyphProps {
  code: string // "wP".."bK"
}

// Piece art: transparent PNGs from public/pieces/<code>.png
// (wP..bK, 256x256). Text-glyph fallback lives in git history.
export function PieceGlyph({ code }: PieceGlyphProps) {
  return (
    <img
      className="piece-img"
      src={`/pieces/${code}.png`}
      alt={code}
      draggable={false}
    />
  )
}

export interface AnimMove {
  key: string // unique per move (SAN + ply), remounts the ghost
  code: string // mover art from the pre-move board (pawn pre-promotion)
  from: string
  to: string
  rookCode?: string
  rookFrom?: string
  rookTo?: string
  back?: boolean // reversed takeback slide: also hide the origin square
}

// Sliding ghost: positioned on the origin square, then translated to the
// destination in square units. Mounts per-move (keyed), so rapid moves
// replace mid-flight ghosts instead of queuing.
function GhostPiece({
  code,
  from,
  to,
  flipped,
  onDone,
}: {
  code: string
  from: string
  to: string
  flipped: boolean
  onDone(): void
}) {
  const a = sqDisplay(from, flipped)
  const b = sqDisplay(to, flipped)
  const [go, setGo] = useState(false)
  const doneRef = useRef(onDone)
  doneRef.current = onDone
  useEffect(() => {
    // Double rAF: paint at origin first, then transition.
    let raf2 = 0
    const raf1 = requestAnimationFrame(() => {
      raf2 = requestAnimationFrame(() => setGo(true))
    })
    const t = setTimeout(() => doneRef.current(), 300)
    return () => {
      cancelAnimationFrame(raf1)
      cancelAnimationFrame(raf2)
      clearTimeout(t)
    }
  }, [from, to, code])
  return (
    <div
      className="anim-ghost"
      style={{
        left: `${a.x * 12.5}%`,
        top: `${a.y * 12.5}%`,
        transform: go
          ? `translate(${(b.x - a.x) * 100}%, ${(b.y - a.y) * 100}%)`
          : 'translate(0, 0)',
      }}
      onTransitionEnd={() => doneRef.current()}
    >
      <img
        className="piece-img"
        src={`/pieces/${code}.png`}
        alt={code}
        draggable={false}
      />
    </div>
  )
}

export interface BoardProps {
  board: (string | null)[]
  turn: 'w' | 'b'
  selected: string | null
  targets: LegalTarget[]
  lastFrom: string | null
  lastTo: string | null
  checkKing: string | null // square of checked king, if any
  hotSquares: string[] // hanging pieces (orange), computed server-side
  pinnedSquares: string[] // pinned to king (gray), computed server-side
  silverSquares: string[] // pinned to pricier piece (silver), computed server-side
  anim: AnimMove | null
  flipped: boolean // Black's perspective when true (2P hotseat + future AI)
  drag: { code: string; x: number; y: number } | null // held piece, board-% pos
  dragFrom: string | null // origin square whose static piece hides mid-drag
  interactive: boolean
  onSquare(sq: string): void
  onPress(sq: string, e: React.PointerEvent): void
  onDragMove(e: React.PointerEvent): void
  onDragEnd(e: React.PointerEvent | null): void
  onBoardClick(): void
  onAnimDone(): void
}

export function sqToIndex(sq: string): number {
  const f = sq.charCodeAt(0) - 97
  const r = sq.charCodeAt(1) - 49
  return r * 8 + f
}

export function indexToSq(i: number): string {
  return String.fromCharCode(97 + (i % 8)) + String.fromCharCode(49 + Math.floor(i / 8))
}

// Display coords (0..7 left→right, top→bottom) for a square name.
// Normal: White at the bottom. Flipped: board rotated 180° (Black's view).
export function sqDisplay(sq: string, flipped: boolean): { x: number; y: number } {
  const f = sq.charCodeAt(0) - 97
  const r = sq.charCodeAt(1) - 49
  return flipped ? { x: 7 - f, y: r } : { x: f, y: 7 - r }
}

function findKing(board: (string | null)[], turn: 'w' | 'b'): string | null {
  const want = turn === 'w' ? 'wK' : 'bK'
  const i = board.indexOf(want)
  return i < 0 ? null : indexToSq(i)
}

// One square: memoized on primitives only, so pointermove re-renders
// (new drag ghost position each frame) skip all 64 squares and only
// paint the ghost. Handlers close over `sq` inside, keeping memo
// comparisons free of fresh closures.
const Square = memo(function Square({
  sq,
  code,
  isLight,
  overlay,
  rankLabel,
  fileLabel,
  dot,
  ring,
  interactive,
  live,
}: {
  sq: string
  code: string | null
  isLight: boolean
  overlay: string | undefined
  rankLabel: number | null
  fileLabel: string | null
  dot: boolean
  ring: boolean
  interactive: boolean
  live: React.MutableRefObject<{
    onSquare(sq: string): void
    onPress(sq: string, e: React.PointerEvent): void
  }>
}) {
  return (
    <div
      className="square"
      onClick={() => interactive && live.current.onSquare(sq)}
      onPointerDown={(e) => interactive && live.current.onPress(sq, e)}
      style={{
        backgroundColor: isLight ? theme.light : theme.dark,
        cursor: interactive ? 'pointer' : 'default',
      }}
    >
      {overlay && <div className="overlay" style={{ backgroundColor: overlay }} />}
      {rankLabel !== null && (
        <span
          className="coord rank"
          style={{ color: isLight ? theme.coordLight : theme.coordDark }}
        >
          {rankLabel}
        </span>
      )}
      {fileLabel !== null && (
        <span
          className="coord file"
          style={{ color: isLight ? theme.coordLight : theme.coordDark }}
        >
          {fileLabel}
        </span>
      )}
      {code && <PieceGlyph code={code} />}
      {dot && <div className="dot" style={{ backgroundColor: theme.dot }} />}
      {ring && <div className="ring" style={{ borderColor: theme.ring }} />}
    </div>
  )
})

export default function Board(props: BoardProps) {
  const { board, selected, targets, lastFrom, lastTo, interactive, anim, flipped } = props
  // Latest handlers: memoized squares keep this ref (stable identity)
  // and always call through to the current render's callbacks.
  const live = useRef({ onSquare: props.onSquare, onPress: props.onPress })
  live.current = { onSquare: props.onSquare, onPress: props.onPress }
  const targetMap = new Map(targets.map((t) => [t.to, t]))
  const checkSq = props.checkKing ?? null
  const hot = new Set(props.hotSquares ?? [])
  const pinned = new Set(props.pinnedSquares ?? [])
  const silver = new Set(props.silverSquares ?? [])
  // While a ghost slides, hide the static pieces on its destinations
  // (captures vanish instantly by design). Reversed takebacks also hide
  // the origin square; held-drag pieces hide their origin the same way.
  const hidden = new Set<string>()
  if (anim) {
    hidden.add(anim.to)
    if (anim.back) hidden.add(anim.from)
    if (anim.rookTo) hidden.add(anim.rookTo)
  }
  if (props.dragFrom) hidden.add(props.dragFrom)

  const rows = []
  for (let dy = 0; dy < 8; dy++) {
    const cells = []
    for (let dx = 0; dx < 8; dx++) {
      const f = flipped ? 7 - dx : dx
      const r = flipped ? dy : 7 - dy
      const sq = indexToSq(r * 8 + f)
      const code = board[r * 8 + f]
      const isLight = (r + f) % 2 === 1
      const isSelected = selected === sq
      const isLast = sq === lastFrom || sq === lastTo
      const isCheck = sq === checkSq
      const isHot = hot.has(sq)
      const isPinned = pinned.has(sq)
      const isSilver = silver.has(sq)
      const target = targetMap.get(sq)

      // Red check > orange hot > gray pinned > silver > yellow last-move.
      // Hot never changes color; silver shows only when not hanging or
      // king-pinned.
      let overlay: string | undefined
      if (isCheck) overlay = theme.check
      else if (isHot) overlay = theme.hot
      else if (isPinned) overlay = theme.pinned
      else if (isSilver) overlay = theme.silver
      else if (isLast) overlay = theme.lastMove
      else if (isSelected) overlay = theme.selected

      cells.push(
        <Square
          key={sq}
          sq={sq}
          code={hidden.has(sq) ? null : code}
          isLight={isLight}
          overlay={overlay}
          rankLabel={dx === 0 ? r + 1 : null}
          fileLabel={dy === 7 ? String.fromCharCode(97 + f) : null}
          dot={!!target && !code}
          ring={!!target && !!code}
          interactive={interactive}
          live={live}
        />,
      )
    }
    rows.push(
      <div key={dy} className="rank-row">
        {cells}
      </div>,
    )
  }
  return (
    <div
      className="board"
      onPointerMove={(e) => interactive && props.onDragMove(e)}
      onPointerUp={(e) => props.onDragEnd(e)}
      onPointerCancel={() => props.onDragEnd(null)}
      onClick={() => {
        // Backstop: with pointer capture the trailing click lands here
        // instead of a square — never let suppression leak into the
        // next gesture.
        props.onBoardClick()
      }}
    >
      {rows}
      {props.drag && (
        <div className="anim-layer">
          <div
            className="anim-ghost drag-ghost"
            style={{
              left: `${props.drag.x - 6.25}%`,
              top: `${props.drag.y - 6.25}%`,
              transform: 'translate(0, 0)',
            }}
          >
            <img
              className="piece-img"
              src={`/pieces/${props.drag.code}.png`}
              alt={props.drag.code}
              draggable={false}
            />
          </div>
        </div>
      )}
      {anim && (
        <div className="anim-layer">
          <GhostPiece
            key={anim.key + '-m'}
            code={anim.code}
            from={anim.from}
            to={anim.to}
            flipped={flipped}
            onDone={props.onAnimDone}
          />
          {anim.rookCode && anim.rookFrom && anim.rookTo && (
            <GhostPiece
              key={anim.key + '-r'}
              code={anim.rookCode}
              from={anim.rookFrom}
              to={anim.rookTo}
              flipped={flipped}
              onDone={props.onAnimDone}
            />
          )}
        </div>
      )}
    </div>
  )
}

export { findKing }
