import { useEffect, useRef } from 'react'

export interface MoveHistoryProps {
  moves: string[]
  livePly: number // moves.length of the live game
  viewedPly: number | null // null = live
  onSelect(ply: number | null): void // null = back to live
}

export default function MoveHistory({ moves, livePly, viewedPly, onSelect }: MoveHistoryProps) {
  const ref = useRef<HTMLDivElement>(null)
  useEffect(() => {
    const el = ref.current
    if (el) el.scrollTop = el.scrollHeight
  }, [moves.length])

  const pairs: { n: number; w: string; wPly: number; b?: string; bPly?: number }[] = []
  for (let i = 0; i < moves.length; i += 2) {
    pairs.push({ n: i / 2 + 1, w: moves[i], wPly: i + 1, b: moves[i + 1], bPly: i + 2 })
  }

  function cellCls(ply: number | undefined): string {
    if (ply === undefined) return ''
    if (viewedPly !== null) return viewedPly === ply ? 'history-viewed' : 'history-clickable'
    return ply === livePly ? 'history-latest history-clickable' : 'history-clickable'
  }

  function pick(ply: number | undefined) {
    if (ply === undefined) return
    // Clicking the viewed (or latest-live) move returns to live.
    onSelect(viewedPly === ply || (viewedPly === null && ply === livePly) ? null : ply)
  }

  return (
    <div className="history-wrap">
      <div className="history-head">
        <span>Moves</span>
        {viewedPly !== null ? (
          <button className="link history-now" onClick={() => onSelect(null)}>
            ● Now
          </button>
        ) : (
          <span className="history-live">live</span>
        )}
      </div>
      <div className="history" ref={ref}>
        {pairs.length === 0 && <div className="history-empty">No moves yet</div>}
        {pairs.map((p) => (
          <div key={p.n} className="history-row">
            <span className="history-num">{p.n}.</span>
            <span className={cellCls(p.wPly)} onClick={() => pick(p.wPly)}>
              {p.w}
            </span>
            <span className={cellCls(p.bPly)} onClick={() => pick(p.bPly)}>
              {p.b ?? ''}
            </span>
          </div>
        ))}
      </div>
    </div>
  )
}
