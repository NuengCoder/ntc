import { useCallback, useEffect, useRef, useState } from 'react'
import {
  getMoves,
  getPosition,
  getState,
  postDraw,
  postMove,
  postNew,
  postRematch,
  postResign,
  postUndo,
} from '../api'
import { playSfx } from '../sfx'
import { TERMINAL_STATUS, type GameState, type LegalTarget, type PositionPreview } from '../types'
import Board, { findKing, sqToIndex, indexToSq, PieceGlyph, type AnimMove } from './Board'
import BackButton from './BackButton'
import MoveHistory from './MoveHistory'

const TC_OPTIONS = ['1+0', '3+2', '5+0', '10+0', '15+10', '30+0', 'unlimited']

// Standard material values (kings excluded — priceless).
const PIECE_VALUE: Record<string, number> = { P: 1, N: 3, B: 3, R: 5, Q: 9 }

// Captured-tray letters (bare kinds, e.g. "Q") → full image codes in
// Q-first value order. `color` is the captured side: White's tray holds
// black pieces and vice versa.
function trayCodes(letters: string[], color: 'w' | 'b'): string[] {
  return letters
    .filter((l) => PIECE_VALUE[l] !== undefined)
    .sort((a, b) => PIECE_VALUE[b] - PIECE_VALUE[a])
    .map((l) => `${color}${l}`)
}

// White-minus-black material from the live board (promotion-proof:
// counts what's on the board, not the trays).
function materialDiff(board: (string | null)[]): number {
  let d = 0
  for (const c of board) {
    if (!c || c.length < 2) continue
    const v = PIECE_VALUE[c[1]] ?? 0
    d += c[0] === 'w' ? v : -v
  }
  return d
}

function fmtClock(ms: number | null): string {
  if (ms === null) return '∞'
  const total = Math.max(0, Math.ceil(ms / 1000))
  const m = Math.floor(total / 60)
  const s = total % 60
  return `${m}:${s.toString().padStart(2, '0')}`
}

function statusText(s: GameState): string {
  switch (s.status) {
    case 'ongoing':
      return `${s.turn === 'w' ? 'White' : 'Black'} to move`
    case 'check':
      return `${s.turn === 'w' ? 'White' : 'Black'} to move — check!`
    case 'checkmate':
      return `Checkmate — ${s.winner === 'white' ? 'White' : 'Black'} wins`
    case 'stalemate':
      return 'Stalemate — draw'
    case 'fifty-move':
      return 'Draw — fifty-move rule'
    case 'repetition':
      return 'Draw — threefold repetition'
    case 'insufficient-material':
      return 'Draw — insufficient material'
    case 'agreement':
      return 'Draw — by agreement'
    case 'resign':
      return `${s.winner === 'white' ? 'White' : 'Black'} wins — resignation`
    case 'timeout':
      return `${s.winner === 'white' ? 'White' : 'Black'} wins — time`
    default:
      return s.status
  }
}

// Build a slide-ghost from a known board (mover art + castling rook hop).
// Shared by the optimistic click path and the server-reconcile fallback.
function buildAnim(
  board: (string | null)[],
  from: string,
  to: string,
  key: string,
  reversed = false,
): AnimMove {
  const mover = board[sqToIndex(from)] ?? 'wP'
  let rook: { code: string; from: string; to: string } | null = null
  const ff = from.charCodeAt(0) - 97
  const tf = to.charCodeAt(0) - 97
  const rank = from[1]
  if ((mover === 'wK' || mover === 'bK') && Math.abs(tf - ff) === 2) {
    // Side by landing file either way (forward O-O lands on g,
    // reversed takeback starts from g).
    const kingside = Math.max(ff, tf) === 6
    const home = (kingside ? 'h' : 'a') + rank
    const away = (kingside ? 'f' : 'd') + rank
    const rf = reversed ? away : home
    const rt = reversed ? home : away
    rook = {
      code: board[sqToIndex(rf)] ?? (mover[0] + 'R'),
      from: rf,
      to: rt,
    }
  }
  return {
    key,
    code: mover,
    from,
    to,
    rookCode: rook?.code,
    rookFrom: rook?.from,
    rookTo: rook?.to,
  }
}

export default function GameScreen({ onExit }: { onExit(): void }) {
  const [state, setState] = useState<GameState | null>(null)
  const [selected, setSelected] = useState<string | null>(null)
  const [targets, setTargets] = useState<LegalTarget[]>([])
  const [promo, setPromo] = useState<{ from: string; to: string } | null>(null)
  const [tc, setTc] = useState('10+0')
  const [notice, setNotice] = useState<string | null>(null)
  const [confirmResign, setConfirmResign] = useState(false)
  const [anim, setAnim] = useState<AnimMove | null>(null)
  const stateRef = useRef<GameState | null>(null)
  stateRef.current = state
  const animRef = useRef<AnimMove | null>(null)
  animRef.current = anim
  // Set by the undo handler so the san effect keeps our reversed ghost
  // instead of rebuilding a forward one from the rewound state.
  const justUndidRef = useRef(false)
  // Board orientation (view-only). Local so each device keeps its own
  // perspective; a server setting would force both sides to share it.
  const [flipped, setFlipped] = useState(() => {
    try {
      return localStorage.getItem('ntc.chess.flip') === '1'
    } catch {
      return false
    }
  })
  // Unconfirmed optimistic move (ghost sliding ahead of the POST).
  // Cleared when its response lands (match) or fails (snap back).
  const pendingRef = useRef<{ from: string; to: string } | null>(null)
  const animSeq = useRef(0)
  // Pre-move board, captured before each state swap so the slide ghost
  // uses the mover's art (pawn pre-promotion, king pre-castle).
  const prevBoard = useRef<(string | null)[] | null>(null)
  // In-flight poll fetch, aborted on every mutation so a stale poll can
  // never resolve after our own POST and replay its ghost. Also guards
  // fetch pile-up when a poll stalls past the 1s interval.
  const pollCtrl = useRef<AbortController | null>(null)
  // Post-reset quiet window: new/rematch rewind move count, so the
  // monotonic guards below can't tell stale from fresh — skip polls
  // briefly instead (resets are rare and user-initiated).
  const suppressPollUntil = useRef(0)
  // Time-travel preview: viewed ply (null = live) + its frozen snapshot.
  // View-only overlay: clocks keep ticking from live state, and the board
  // locks (no moves) until Now.
  const [viewPly, setViewPly] = useState<number | null>(null)
  const [preview, setPreview] = useState<PositionPreview | null>(null)
  const previewPly = useRef<number | null>(null)

  // A poll response is stale when it rewinds the move count (pre-move
  // read resolving late) or un-terminates a finished game (pre-resign /
  // pre-draw read). Hotseat invariants: move count only grows, terminal
  // states never revert — except via new/rematch, covered by the quiet
  // window above.
  function isStalePoll(incoming: GameState): boolean {
    const cur = stateRef.current
    if (!cur) return false
    const nl = incoming.moves?.length ?? 0
    const cl = cur.moves?.length ?? 0
    if (nl < cl) return true
    if (
      nl === cl &&
      TERMINAL_STATUS.has(cur.status) &&
      !TERMINAL_STATUS.has(incoming.status)
    )
      return true
    return false
  }

  function markMutating() {
    pollCtrl.current?.abort()
    pollCtrl.current = null
  }

  const refresh = useCallback(async () => {
    if (Date.now() < suppressPollUntil.current) return
    pollCtrl.current?.abort()
    const ctrl = new AbortController()
    pollCtrl.current = ctrl
    try {
      const ns = await getState(ctrl.signal)
      if (!isStalePoll(ns)) setState(ns)
    } catch {
      /* aborted or server hiccup — keep last state, next poll retries */
    } finally {
      if (pollCtrl.current === ctrl) pollCtrl.current = null
    }
  }, [])

  useEffect(() => {
    refresh()
    let id = window.setInterval(refresh, 1000)    // Background tabs: stop polling entirely (zero traffic/battery);
    // refetch immediately on return. Foreground keeps the 1s cadence
    // (tunnel users feel every round-trip, so no wasted fetches).
    const onVis = () => {
      window.clearInterval(id)
      if (!document.hidden) {
        refresh()
        id = window.setInterval(refresh, 1000)
      }
    }
    document.addEventListener('visibilitychange', onVis)
    return () => {
      window.clearInterval(id)
      document.removeEventListener('visibilitychange', onVis)
      pollCtrl.current?.abort()
      cancelDragFrame()
    }
  }, [refresh])

  // If selection becomes stale (opponent... same-screen turn change), clear it.
  useEffect(() => {
    setSelected(null)
    setTargets([])
  }, [state?.last_move?.san])

  // Slide animation: optimistic ghosts start on click (buildAnim below);
  // this effect only reconciles server truth — a matching arrival reveals
  // the confirmed static piece, anything else is left to the ghost.
  // Keyed uniquely so rapid moves replace mid-flight ghosts.
  useEffect(() => {
    const lm = state?.last_move
    const pend = pendingRef.current
    if (justUndidRef.current) {
      justUndidRef.current = false
      if (state) prevBoard.current = state.board
      return
    }
    if (lm && pend && pend.from === lm.from && pend.to === lm.to) {
      // Our optimistic ghost is confirmed: reveal the static piece
      // (pixel-identical handoff, no replay).
      pendingRef.current = null
      setAnim(null)
    } else if (!pend) {
      // Server-driven move we didn't initiate (vanishingly rare hotseat —
      // own moves always go through doMove first). Never animate into a
      // finished game (e.g. resign landing mid-flight).
      if (lm && prevBoard.current && !TERMINAL_STATUS.has(state?.status ?? '')) {
        setAnim(
          buildAnim(
            prevBoard.current,
            lm.from,
            lm.to,
            `${state?.moves.length ?? 0}:${lm.san}`,
          ),
        )
      } else {
        setAnim(null)
      }
    }
    if (state) prevBoard.current = state.board
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state?.last_move?.san])

  // Time-travel preview (view-only, never a mutation): fetch the frozen
  // board for a ply and lock the board until Now. Latest-or-null returns
  // to live. Stale fetches (rapid clicks) are dropped by ply match.
  async function selectPly(ply: number | null) {
    if (ply === null) {
      previewPly.current = null
      setPreview(null)
      setViewPly(null)
      return
    }
    const cur = stateRef.current
    if (!cur || ply < 1 || ply > cur.moves.length) return
    if (previewPly.current === ply) return
    previewPly.current = ply
    try {
      const pv = await getPosition(ply)
      if (previewPly.current !== ply) return
      setPreview(pv)
      setViewPly(ply)
      setAnim(null)
      setSelected(null)
      setTargets([])
      setConfirmResign(false)
    } catch (e) {
      if (previewPly.current === ply) previewPly.current = null
      setNotice(e instanceof Error ? e.message : 'Preview failed')
    }
  }

  function toggleFlip() {
    setFlipped((f) => {
      const next = !f
      try {
        localStorage.setItem('ntc.chess.flip', next ? '1' : '0')
      } catch {
        /* private mode — session-only */
      }
      return next
    })
  }

  async function doUndoOnce(): Promise<boolean> {
    const cur = stateRef.current
    if (!cur || cur.moves.length === 0) return false
    try {
      const ns = await postUndo()
      if (ns.error || !ns.undone) {
        if (!ns.error) setNotice('Nothing to undo')
        return false
      }
      // Reversed slide: buildAnim from the pre-undo board yields the
      // mover art at `to` plus the reversed rook hop for castling
      // takebacks. `back` also hides the origin square mid-flight.
      animSeq.current += 1
      justUndidRef.current = true
      setAnim({
        ...buildAnim(
          cur.board,
          ns.undone.to,
          ns.undone.from,
          `undo-${ns.moves.length}-${ns.undone.san}`,
          true,
        ),
        back: true,
      })
      setState(ns)
      setSelected(null)
      setTargets([])
      return true
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Undo failed')
      return false
    }
  }

  // Hold-to-rewind undo: pointerdown does one takeback immediately and
  // repeats every 320ms (each slide is 140ms); release stops. Keyboard
  // users get single undos via onClick (pointerdown preventDefault
  // suppresses the synthetic mouse/touch click, so no double-fire).
  const holdTimer = useRef<number | null>(null)
  const holdFlight = useRef(false)

  function stopHoldUndo() {
    if (holdTimer.current !== null) {
      clearInterval(holdTimer.current)
      holdTimer.current = null
    }
  }

  async function undoTick() {
    if (holdFlight.current) return
    // A move POST is still unconfirmed: skip this tick (don't stop the
    // hold) so undo can never interleave mid-flight and desync the ghost.
    if (pendingRef.current) return
    const cur = stateRef.current
    if (!cur || cur.moves.length === 0) {
      stopHoldUndo()
      return
    }
    holdFlight.current = true
    try {
      const ok = await doUndoOnce()
      if (!ok) stopHoldUndo()
    } finally {
      holdFlight.current = false
    }
  }

  function startHoldUndo(e: React.PointerEvent) {
    e.preventDefault()
    stopHoldUndo()
    void undoTick()
    holdTimer.current = window.setInterval(() => {
      void undoTick()
    }, 320)
  }

  // Hold-drag session (ref: no re-render churn in handlers).
  const dragSess = useRef<{
    from: string
    wasSelected: boolean
    boardEl: HTMLElement
    sx: number
    sy: number
    moved: boolean
  } | null>(null)
  const [dragPos, setDragPos] = useState<{ code: string; from: string; x: number; y: number } | null>(null)
  // Latest pointer position + queued frame: caps ghost updates at one
  // render per frame on 120Hz touchscreens.
  const dragLatest = useRef<{ code: string; from: string; x: number; y: number } | null>(null)
  const dragRaf = useRef(0)

  function flushDrag() {
    dragRaf.current = 0
    const latest = dragLatest.current
    // A drop/unmount may have landed mid-frame: never resurrect.
    if (latest && dragSess.current) setDragPos(latest)
  }

  function cancelDragFrame() {
    if (dragRaf.current) {
      cancelAnimationFrame(dragRaf.current)
      dragRaf.current = 0
    }
    dragLatest.current = null
  }
  // Set after any pointer-driven drop so the trailing synthetic click
  // never re-fires handleSquare (e.g. re-selecting a just-dropped piece).
  const suppressClick = useRef(false)

  function boardPct(boardEl: HTMLElement, clientX: number, clientY: number) {
    const r = boardEl.getBoundingClientRect()
    const x = Math.min(100, Math.max(0, ((clientX - r.left) / r.width) * 100))
    const y = Math.min(100, Math.max(0, ((clientY - r.top) / r.height) * 100))
    return { x, y }
  }

  function hitSquare(boardEl: HTMLElement, clientX: number, clientY: number): string | null {
    const r = boardEl.getBoundingClientRect()
    const dx = Math.floor(((clientX - r.left) / r.width) * 8)
    const dy = Math.floor(((clientY - r.top) / r.height) * 8)
    if (dx < 0 || dx > 7 || dy < 0 || dy > 7) return null
    return indexToSq((flipped ? dy : 7 - dy) * 8 + (flipped ? 7 - dx : dx))
  }

  function clearSelection() {
    setSelected(null)
    setTargets([])
  }

  async function selectPiece(sq: string) {
    setSelected(sq)
    try {
      const r = await getMoves(sq)
      setTargets(r.moves)
    } catch {
      setTargets([])
    }
  }

  function onPress(sq: string, e: React.PointerEvent) {
    // Fresh press clears any orphaned suppression (e.g. a click lost
    // between pointerup and compat-mouse dispatch).
    suppressClick.current = false
    const s = stateRef.current
    if (!s || TERMINAL_STATUS.has(s.status) || promo || viewPly !== null) return
    if (e.button !== undefined && e.button !== 0 && e.pointerType === 'mouse') return
    const code = s.board[sqToIndex(sq)]
    if (!code || code[0] !== s.turn) return
    if (sq !== selected) void selectPiece(sq)
    const boardEl = (e.currentTarget as HTMLElement).closest('.board') as HTMLElement | null
    if (!boardEl) return
    try {
      boardEl.setPointerCapture(e.pointerId)
    } catch {
      /* older browsers: tracking degrades gracefully */
    }
    const p = boardPct(boardEl, e.clientX, e.clientY)
    dragSess.current = { from: sq, wasSelected: selected === sq, boardEl, sx: p.x, sy: p.y, moved: false }
    setDragPos({ code, from: sq, x: p.x, y: p.y })
  }

  function onDragMove(e: React.PointerEvent) {
    const sess = dragSess.current
    if (!sess) return
    const s = stateRef.current
    if (!s) return
    const code = s.board[sqToIndex(sess.from)]
    if (!code) {
      dragSess.current = null
      cancelDragFrame()
      setDragPos(null)
      return
    }
    const p = boardPct(sess.boardEl, e.clientX, e.clientY)
    if (!sess.moved && Math.hypot(p.x - sess.sx, p.y - sess.sy) > 3) sess.moved = true
    dragLatest.current = { code, from: sess.from, x: p.x, y: p.y }
    if (!dragRaf.current) dragRaf.current = requestAnimationFrame(flushDrag)
  }

  function onDragEnd(e: React.PointerEvent | null) {
    const sess = dragSess.current
    dragSess.current = null
    cancelDragFrame()
    setDragPos(null)
    if (!sess) return
    // Cancelled mid-gesture (touch interruption): no click follows, so
    // no suppression needed — just snap back + deselect.
    if (e === null) {
      clearSelection()
      return
    }
    // A real pointerup is always followed by a click: suppress it so
    // handleSquare never re-fires (with pointer capture the click lands
    // on the board root, whose clearer below is the backstop). The flag
    // goes up only AFTER the direct call below — setting it first would
    // make handleSquare eat its own invocation and the drop would never
    // move (every legal drop silently dies).
    const hit = hitSquare(sess.boardEl, e.clientX, e.clientY)
    if (!sess.moved) {
      // Tap (no real travel): toggle — re-pressing a selected piece
      // deselects, pressing fresh keeps the new selection.
      suppressClick.current = true
      if (hit === sess.from && sess.wasSelected) clearSelection()
      return
    }
    if (hit === null || hit === sess.from) {
      // Dragged home or off-board: snap back + deselect.
      suppressClick.current = true
      clearSelection()
      return
    }
    void handleSquare(hit)
    suppressClick.current = true
  }

  // Shared target routing for taps and drops: fetch-then-route so a
  // press that outruns the dots fetch (routine over a tunnel) still
  // moves instead of misreading empty targets as illegal.
  async function commitTo(from: string, to: string, cached: LegalTarget[]) {
    let moves = cached
    if (moves.length === 0) {
      try {
        moves = (await getMoves(from)).moves
      } catch {
        clearSelection()
        return
      }
    }
    const target = moves.find((t) => t.to === to)
    if (!target) {
      // Not a target: tapping another own piece switches selection,
      // anything else deselects (drop path passes only real targets
      // here via handleSquare, so this is the tap-switch/deselect).
      const s = stateRef.current
      const code = s?.board[to.charCodeAt(0) - 97 + (to.charCodeAt(1) - 49) * 8]
      if (s && code && code[0] === s.turn && to !== from) {
        await selectPiece(to)
      } else {
        clearSelection()
      }
      return
    }
    if (target.promo) {
      setPromo({ from, to })
      return
    }
    await doMove(from, to)
  }

  // Set after any pointer-driven drop so the trailing synthetic click
  // never re-fires handleSquare (e.g. re-selecting a just-dropped piece).
  // (Declared once, beside the drag session above.)
  async function handleSquare(sq: string) {
    if (suppressClick.current) {
      suppressClick.current = false
      return
    }
    const s = stateRef.current
    if (!s || TERMINAL_STATUS.has(s.status) || promo) return
    const code = s.board[sq.charCodeAt(0) - 97 + (sq.charCodeAt(1) - 49) * 8]
    // Click a target square → move there.
    if (selected) {
      await commitTo(selected, sq, targets)
      return
    }
    // Click own-turn piece → select + dots (re-pressing selected
    // deselects; the pointer layer normally handles this first).
    if (code && code[0] === s.turn) {
      if (sq === selected) {
        clearSelection()
        return
      }
      await selectPiece(sq)
    } else {
      clearSelection()
    }
  }

  async function doMove(from: string, to: string, p?: string) {
    setPromo(null)
    markMutating()
    // Optimistic ghost: slide now from the current board, reconcile when
    // the POST lands (match → reveal, error → snap back). Unconfirmed
    // ghosts hold at the destination instead of clearing.
    const cur = stateRef.current
    if (cur) {
      animSeq.current += 1
      pendingRef.current = { from, to }
      setAnim(buildAnim(cur.board, from, to, `opt-${animSeq.current}-${from}-${to}`))
    }
    try {
      const ns = await postMove(from, to, p)
      if (ns.error) {
        pendingRef.current = null
        setAnim(null)
        setNotice(`Illegal move (${ns.error})`)
        return
      }
      if (ns.sfx) playSfx(ns.sfx === 'win' ? 'win' : ns.sfx)
      if (ns.abandoned) setNotice('Previous game abandoned')
      setState(ns)
      setSelected(null)
      setTargets([])
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Move failed')
    }
  }

  async function newGame() {
    markMutating()
    try {
      const ns = await postNew(tc)
      if (ns.sfx) playSfx(ns.sfx)
      if (ns.abandoned) setNotice('Previous game abandoned — side to move forfeits')
      setState(ns)
      setSelected(null)
      setTargets([])
      setPromo(null)
      setConfirmResign(false)
      suppressPollUntil.current = Date.now() + 1200
      pendingRef.current = null
      setAnim(null)
      previewPly.current = null
      setPreview(null)
      setViewPly(null)
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'New game failed')
    }
  }

  async function rematch() {
    markMutating()
    try {
      const ns = await postRematch()
      setState(ns)
      setSelected(null)
      setTargets([])
      setPromo(null)
      setConfirmResign(false)
      setNotice(null)
      suppressPollUntil.current = Date.now() + 1200
      pendingRef.current = null
      setAnim(null)
      previewPly.current = null
      setPreview(null)
      setViewPly(null)
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Rematch failed')
    }
  }

  async function resign() {
    const s = stateRef.current
    if (!s) return
    markMutating()
    try {
      const ns = await postResign(s.turn === 'w' ? 'white' : 'black')
      if (!ns.error) playSfx('lose')
      setState(ns.error ? s : ns)
      setConfirmResign(false)
      pendingRef.current = null
      setAnim(null)
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Resign failed')
    }
  }

  async function draw() {
    markMutating()
    try {
      const ns = await postDraw()
      setState(ns.error ? (stateRef.current as GameState) : ns)
      pendingRef.current = null
      setAnim(null)
    } catch (e) {
      setNotice(e instanceof Error ? e.message : 'Draw failed')
    }
  }

  if (!state) return <div className="loading">Connecting to chess server…</div>
  const over = TERMINAL_STATUS.has(state.status)
  const checkSq = state.in_check ? findKing(state.board, state.turn) : null
  // Time-travel preview: frozen board/trays/last-move from the snapshot,
  // live clocks/score context. Overlays that need live evaluation
  // (check, hot, pinned, silver) stay dark in preview.
  const isPreview = viewPly !== null && preview !== null
  const showBoard = isPreview && preview ? preview.board : state.board
  const showLast = isPreview && preview ? preview.last_move : state.last_move
  const showCapW = isPreview && preview ? preview.captured_by_white : state.captured_by_white
  const showCapB = isPreview && preview ? preview.captured_by_black : state.captured_by_black
  const diff = materialDiff(showBoard)
  const viewedLabel =
    viewPly !== null && preview?.last_move
      ? `Viewing ${Math.ceil(viewPly / 2)}. ${preview.last_move.san}`
      : null

  return (
    <div className="game">
      <div className="game-top">
        <BackButton onBack={onExit} />
        <span className="tc-label">{state.clocks.tc}</span>
        <button className="back-btn" onClick={toggleFlip} title={flipped ? "White at bottom" : "Black at bottom"}>
          ⇄ Flip
        </button>
      </div>

      <div className="game-main">
        <div className="board-col">
          <div className="clock-row">
            <span className="tray">
              {trayCodes(showCapB, 'w').map((code, i) => (
                <PieceGlyph key={`${code}-${i}`} code={code} />
              ))}
            </span>
            <span className={state.turn === 'b' && !over ? 'clock active' : 'clock'}>
              ♟ black {fmtClock(state.clocks.b)}
              {diff < 0 && <span className="score">+{-diff}</span>}
            </span>
          </div>

          <Board
            board={showBoard}
            turn={state.turn}
            selected={selected}
            targets={targets}
            lastFrom={showLast?.from ?? null}
            lastTo={showLast?.to ?? null}
            checkKing={isPreview ? null : checkSq}
            hotSquares={isPreview ? [] : (state.hot ?? [])}
            pinnedSquares={isPreview ? [] : (state.pinned ?? [])}
            silverSquares={isPreview ? [] : (state.silver ?? [])}
            anim={isPreview ? null : anim}
            flipped={flipped}
            drag={dragPos}
            dragFrom={dragPos?.from ?? null}
            interactive={!over && !isPreview}
            onSquare={handleSquare}
            onPress={onPress}
            onDragMove={onDragMove}
            onDragEnd={onDragEnd}
            onBoardClick={() => {
              suppressClick.current = false
            }}
            onAnimDone={() => {
              // Unconfirmed ghosts hold at the destination (no snap-back
              // while the POST is in flight); confirmed ones reveal static.
              const a = animRef.current
              const p = pendingRef.current
              if (a && p && p.from === a.from && p.to === a.to) return
              setAnim(null)
            }}
          />

          <div className="clock-row">
            <span className="tray">
              {trayCodes(showCapW, 'b').map((code, i) => (
                <PieceGlyph key={`${code}-${i}`} code={code} />
              ))}
            </span>
            <span className={state.turn === 'w' && !over ? 'clock active' : 'clock'}>
              ♟ white {fmtClock(state.clocks.w)}
              {diff > 0 && <span className="score">+{diff}</span>}
            </span>
          </div>

          <MoveHistory
            moves={state.moves}
            livePly={state.moves.length}
            viewedPly={viewPly}
            onSelect={selectPly}
          />
        </div>

        <div className="side-col">
          <div className="status">{viewedLabel ?? statusText(state)}</div>
          {notice && (
            <div className="notice" onClick={() => setNotice(null)}>
              {notice}
            </div>
          )}

          <div className="btn-row">
            <select value={tc} onChange={(e) => setTc(e.target.value)} disabled={false}>
              {TC_OPTIONS.map((o) => (
                <option key={o} value={o}>
                  {o}
                </option>
              ))}
            </select>
            <button onClick={newGame}>New</button>
          </div>

          <div className="btn-row">
            <button
              onPointerDown={startHoldUndo}
              onPointerUp={stopHoldUndo}
              onPointerLeave={stopHoldUndo}
              onPointerCancel={stopHoldUndo}
              onClick={() => {
                void undoTick()
              }}
              onContextMenu={(e) => e.preventDefault()}
              disabled={state.moves.length === 0 || promo !== null || isPreview}
              title={isPreview ? "Return to live first" : "Undo last move (hold to rewind)"}
            >
              ↩ Undo
            </button>
          </div>

          {!over && (
            <div className="btn-row">
              {confirmResign && !isPreview ? (
                <>
                  <button className="danger" onClick={resign}>
                    Confirm resign ({state.turn === 'w' ? 'white' : 'black'})
                  </button>
                  <button onClick={() => setConfirmResign(false)}>Cancel</button>
                </>
              ) : (
                <button
                  onClick={() => setConfirmResign(true)}
                  disabled={isPreview}
                  title={isPreview ? 'Return to live first' : undefined}
                >
                  Resign
                </button>
              )}
              <button
                onClick={draw}
                disabled={isPreview}
                title={isPreview ? 'Return to live first' : undefined}
              >
                Draw (agree)
              </button>
            </div>
          )}

          {over && (
            <div className="btn-row">
              <button className="primary" onClick={rematch}>
                Rematch
              </button>
              <button onClick={onExit}>Menu</button>
            </div>
          )}

          <div className="meta">
            <span>halfmove {state.halfmove}</span>
            <span>repetition ×{state.repetition}</span>
          </div>
          {state.last_result && (
            <div className="meta">
              Last: {state.last_result.winner ?? 'draw'} ({state.last_result.reason})
            </div>
          )}
        </div>
      </div>

      {promo && (
        <div className="modal-back">
          <div className="modal">
            <div className="modal-title">Promote to</div>
            <div className="btn-row">
              {['Q', 'R', 'B', 'N'].map((p) => {
                const code = `${state.turn}${p}`
                return (
                  <button
                    key={p}
                    className="promo-btn"
                    title={p}
                    onClick={() => doMove(promo.from, promo.to, p)}
                  >
                    <img src={`/pieces/${code}.png`} alt={p} draggable={false} />
                  </button>
                )
              })}
            </div>
            <button className="link" onClick={() => setPromo(null)}>
              cancel
            </button>
          </div>
        </div>
      )}

      {over && (
        <div className="modal-back">
          <div className="modal">
            <div className="modal-title">{statusText(state)}</div>
            <div className="btn-row">
              <button className="primary" onClick={rematch}>
                Rematch
              </button>
              <button onClick={onExit}>Menu</button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
