import { useState } from 'react'
import BackButton from './BackButton'

const STEPS = [
  {
    title: '1 — Select',
    body: 'Click any piece of the side to move. Legal destination squares appear as mini black dots; captures show as rings.',
  },
  {
    title: '2 — Move',
    body: 'Click a dot to move. Special moves just work: pawn double-push on its first move, en passant, castling (king two squares), promotion picker.',
  },
  {
    title: '3 — Finish',
    body: 'Watch the status line for check, mate and auto-draws. History below the board records every move in SAN. Resign, agree a draw, or hit Rematch.',
  },
]

export default function TutorialScreen({ onBack }: { onBack(): void }) {
  const [i, setI] = useState(0)
  return (
    <div className="screen">
      <BackButton onBack={onBack} />
      <h2>Tutorial</h2>
      <div className="card">
        <h3>{STEPS[i].title}</h3>
        <p>{STEPS[i].body}</p>
      </div>
      <div className="btn-row">
        <button disabled={i === 0} onClick={() => setI(i - 1)}>
          Prev
        </button>
        <span>
          {i + 1} / {STEPS.length}
        </span>
        <button disabled={i === STEPS.length - 1} onClick={() => setI(i + 1)}>
          Next
        </button>
      </div>
    </div>
  )
}
