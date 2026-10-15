import BackButton from './BackButton'

export default function HelpScreen({ onBack }: { onBack(): void }) {
  return (
    <div className="screen">
      <BackButton onBack={onBack} />
      <h2>Help</h2>
      <div className="prose">
        <h3>Playing</h3>
        <p>Click one of your pieces — legal targets show as black dots (rings on captures). Click a dot to move. Pawns promote via the picker. Castle by moving the king two squares; en passant works like normal chess.</p>
        <p>Orange squares are hanging: pieces attacked more than defended (a cheaper piece eyeing a pricier one always glows), plus danger zones around both kings.</p>
        <p>Gray squares are pinned to the king — their attacks only count against the pinner. Aqua squares are pinned to a pricier piece, same rule.</p>
        <p>Captured trays show taken pieces as images (strongest first); <code>+N</code> by a clock is that side's material lead (P1 N3 B3 R5 Q9).</p>
        <p>↩ Undo takes back one ply (hold to rewind); ⇄ Flip swaps the board perspective.</p>
        <p>Click any move in the history to preview that position (view-only — press ● Now to return to live).</p>
        <h3>Notation</h3>
        <p>History uses standard algebraic notation: <code>e4 Nf3 O-O O-O-O Bxf4 Bxf4+ Bxf4#</code> (+ check, # mate).</p>
        <h3>Endings</h3>
        <ul>
          <li>Win: checkmate, opponent resigns, timeout, abandonment.</li>
          <li>Draw: stalemate, threefold repetition (auto), fifty-move (auto), insufficient material (auto), agreement.</li>
        </ul>
        <h3>Clocks</h3>
        <p>Server-side countdowns start after White's first move. Pick a control (1+0 … 30+0, unlimited) before New.</p>
      </div>
    </div>
  )
}
