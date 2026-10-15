// Single home for board colors — the future PNG/animation polish pass
// only needs to touch this file (+ PieceGlyph below).
export const theme = {
  light: '#EBD8B7', // egg
  dark: '#A9714B', // brown
  coordLight: '#A9714B', // label ink on egg squares
  coordDark: '#EBD8B7', // label ink on brown squares
  lastMove: 'rgba(247, 236, 89, 0.65)', // yellow
  check: 'rgba(231, 76, 60, 0.85)', // red
  hot: 'rgba(255, 140, 0, 0.55)', // orange: hanging / weak squares
  pinned: 'rgba(128, 128, 128, 0.55)', // gray: pinned to the king
  silver: 'rgba(0, 200, 200, 0.55)', // aqua: pinned to a pricier piece
  selected: 'rgba(255, 255, 255, 0.25)',
  dot: 'rgba(20, 12, 8, 0.55)', // mini black dot
  ring: 'rgba(20, 12, 8, 0.65)', // capture ring
  whitePiece: '#FAFAFA',
  blackPiece: '#1A1A1A',
}
