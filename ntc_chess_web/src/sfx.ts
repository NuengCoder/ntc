// Sound engine (placeholder-friendly).
//
// Tries `/sounds/<name>.wav` (move, capture, promote, castle, check, win, lose).
// Missing files 404 and fail silently, so dropping the real .wav pack into
// `ntc_chess_web/public/sounds/` later needs zero code changes.
// Volumes 0-100 come from the settings API (sfx.<name> + sfx.master).

let volumes: Record<string, number> = {}

export function setVolumes(v: Record<string, number>) {
  volumes = v
}

function level(name: string): number {
  const master = volumes['sfx.master'] ?? 80
  const per = volumes[`sfx.${name}`] ?? 80
  return (master / 100) * (per / 100)
}

export function playSfx(name: string) {
  try {
    const vol = level(name)
    if (vol <= 0) return
    const a = new Audio(`/sounds/${name}.wav`)
    a.volume = Math.min(1, Math.max(0, vol))
    // 404 / decode errors reject — swallow: assets land later.
    a.play().catch(() => {})
  } catch {
    /* Audio unsupported — silent */
  }
}
