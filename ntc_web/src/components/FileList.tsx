import { useState } from 'react'
import type { FileItem } from '../types'
import { shareUrl, startDownload, getNativeDownloadUrl, type DownloadHandle } from '../api'

interface Props {
  items: FileItem[]
  loading: boolean
  path: string
  onNavigate: (p: string) => void
  onRename: (item: FileItem) => void
  onDelete: (item: FileItem) => void
  onRefresh: () => void
  onError: (msg: string) => void
  readOnly?: boolean
}

type Active = { pct: number; paused: boolean; handle: DownloadHandle }

// Text badges instead of emoji: the extension itself is the icon, so the
// column stays readable in every font/terminal theme.
function fileBadge(name: string, isDir: boolean): string {
  if (isDir) return 'DIR'
  const raw = name.split('.').pop()?.toLowerCase() || ''
  if (!name.includes('.') || raw.length > 4 || raw === '') return 'FILE'
  return raw.toUpperCase()
}

export function FileList({ items, loading, path, onNavigate, onRename, onDelete, onError, readOnly }: Props) {
  const dirs = items.filter(i => i.isDir)
  const files = items.filter(i => !i.isDir)
  // One entry per file being transferred right now: the row shows pause and
  // cancel instead of rename/delete until the transfer ends.
  const [active, setActive] = useState<Record<number, Active>>({})
  const [copied, setCopied] = useState<number | null>(null)

  function fullPath(item: FileItem): string {
    return path === '/' ? `/${item.name}` : `${path}/${item.name}`
  }

  function patch(id: number, next: Partial<Active>) {
    setActive(prev => (prev[id] ? { ...prev, [id]: { ...prev[id], ...next } } : prev))
  }

  function clear(id: number) {
    setActive(prev => {
      const next = { ...prev }
      delete next[id]
      return next
    })
  }

  function handleDownload(item: FileItem) {
    const handle = startDownload(fullPath(item), item.name, pct => patch(item.id, { pct }))
    setActive(prev => ({ ...prev, [item.id]: { pct: 0, paused: false, handle } }))
    handle.done.then(
      () => clear(item.id),
      (e: any) => {
        clear(item.id)
        if (e?.message && e.message !== 'cancelled') onError(e.message)
      },
    )
  }

  function togglePause(item: FileItem) {
    const cur = active[item.id]
    if (!cur) return
    if (cur.paused) {
      cur.handle.resume()
      patch(item.id, { paused: false })
    } else {
      cur.handle.pause()
      patch(item.id, { paused: true })
    }
  }

  function handleCancel(item: FileItem) {
    active[item.id]?.handle.cancel()
    clear(item.id)
  }

  // Browser-native download (download tray progress). The URL is the same
  // stable /download/root/... link the copy button hands out.
  async function handleNativeDownload(item: FileItem) {
    try {
      const url = await getNativeDownloadUrl(fullPath(item), item.id)
      const a = document.createElement('a')
      a.href = url
      a.download = item.name
      a.target = '_blank'
      a.rel = 'noopener'
      document.body.appendChild(a)
      a.click()
      a.remove()
    } catch (e: any) {
      onError(e.message || 'Download failed')
    }
  }

  async function copyLink(item: FileItem) {
    const absolute = new URL(shareUrl(fullPath(item)), window.location.origin).href
    try {
      await navigator.clipboard.writeText(absolute)
      setCopied(item.id)
      setTimeout(() => setCopied(id => (id === item.id ? null : id)), 1500)
    } catch {
      onError('Could not copy the link')
    }
  }

  if (loading) {
    return <div className="loading">Loading…</div>
  }

  if (items.length === 0) {
    return <div className="empty">This folder is empty</div>
  }

  return (
    <div className="file-list">
      {[...dirs, ...files].map(item => {
        const dl = active[item.id]
        return (
          <div key={item.id} className="file-entry">
            <div className="file-row">
              <span className="file-icon">{fileBadge(item.name, item.isDir)}</span>
              <span
                className="file-name"
                onClick={() => item.isDir ? onNavigate(path === '/' ? `/${item.name}` : `${path}/${item.name}`) : null}
              >
                {item.name}
              </span>
              <span className="file-size">
                {item.isDir ? '—' : formatSize(item.size)}
              </span>
              <span className="file-actions">
                {!item.isDir && (dl ? (
                  <>
                    <span className="dl-pct">{dl.pct}%</span>
                    <button
                      className="act-btn"
                      onClick={() => togglePause(item)}
                      title={dl.paused ? 'Resume download' : 'Pause download'}
                    >
                      {dl.paused ? 'Resume' : 'Pause'}
                    </button>
                    <button
                      className="act-btn del"
                      onClick={() => handleCancel(item)}
                      title="Cancel download"
                    >
                      Cancel
                    </button>
                  </>
                ) : (
                  <>
                    <button
                      className="act-btn"
                      onClick={() => handleDownload(item)}
                      title="Download (in-app progress, pausable)"
                    >
                      Save
                    </button>
                    <button
                      className="act-btn"
                      onClick={() => handleNativeDownload(item)}
                      title="Download via browser (native progress)"
                    >
                      Open
                    </button>
                    <button
                      className="act-btn"
                      onClick={() => copyLink(item)}
                      title="Copy link address"
                    >
                      {copied === item.id ? 'Copied' : 'Link'}
                    </button>
                  </>
                ))}
                {(item.isDir || !dl) && (
                  <>
                    <button className="act-btn" onClick={() => onRename(item)} title="Rename" disabled={readOnly} style={readOnly ? { display: 'none' } : undefined}>Rename</button>
                    <button className="act-btn del" onClick={() => onDelete(item)} title="Delete" disabled={readOnly} style={readOnly ? { display: 'none' } : undefined}>Delete</button>
                  </>
                )}
              </span>
            </div>
            {dl && (
              <div className="dl-progress-bar-bg">
                <div className="dl-progress-bar-fill" style={{ width: `${dl.pct}%` }} />
              </div>
            )}
          </div>
        )
      })}
    </div>
  )
}

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1)
  return (bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1) + ' ' + units[i]
}
