import { useState } from 'react'
import type { FileItem } from '../types'
import { downloadFile, getNativeDownloadUrl } from '../api'

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

const extIcons: Record<string, string> = {
  txt: '📝', md: '📝', rtf: '📝',
  js: '🟨', ts: '🟦', py: '🐍', rs: '🦀', go: '🔷',
  java: '☕', kt: '🟣', swift: '🔶',
  html: '🌐', css: '🎨', json: '📋', xml: '📋', yml: '📋', yaml: '📋', toml: '📋',
  png: '🖼', jpg: '🖼', jpeg: '🖼', gif: '🖼', svg: '🖼', webp: '🖼', ico: '🖼',
  mp4: '🎬', avi: '🎬', mkv: '🎬', mov: '🎬',
  mp3: '🎵', wav: '🎵', flac: '🎵', ogg: '🎵',
  pdf: '📕', doc: '📘', docx: '📘', xls: '📗', xlsx: '📗', ppt: '📙', pptx: '📙',
  zip: '📦', tar: '📦', gz: '📦', bz2: '📦', rar: '📦', '7z': '📦',
  exe: '⚙', dmg: '💿', app: '📱', apk: '🤖',
}

function fileIcon(name: string, isDir: boolean): string {
  if (isDir) return '📁'
  const ext = name.split('.').pop()?.toLowerCase() || ''
  return extIcons[ext] || '📄'
}

export function FileList({ items, loading, path, onNavigate, onRename, onDelete, onError, readOnly }: Props) {
  const dirs = items.filter(i => i.isDir)
  const files = items.filter(i => !i.isDir)
  const [downloading, setDownloading] = useState<number | null>(null)
  const [dlProgress, setDlProgress] = useState<Record<number, number>>({})

  // In-app download (Files UI progress bar). Uses XHR + Basic header so it
  // works with and without `serve --auth`.
  async function handleDownload(item: FileItem) {
    setDownloading(item.id)
    setDlProgress(prev => ({ ...prev, [item.id]: 0 }))
    try {
      await downloadFile(item.id, item.name, (pct) => {
        setDlProgress(prev => ({ ...prev, [item.id]: pct }))
      })
    } catch (e: any) {
      onError(e.message || 'Download failed')
    } finally {
      setDownloading(null)
      setDlProgress(prev => {
        const next = { ...prev }
        delete next[item.id]
        return next
      })
    }
  }

  // Browser-native download (download tray progress). Opens a plain link;
  // with --auth a short-lived ?token= is minted so no header is needed.
  async function handleNativeDownload(item: FileItem) {
    try {
      const url = await getNativeDownloadUrl(item.id)
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

  if (loading) {
    return <div className="loading">Loading…</div>
  }

  if (items.length === 0) {
    return <div className="empty">This folder is empty</div>
  }

  return (
    <div className="file-list">
      {[...dirs, ...files].map(item => {
        const pct = dlProgress[item.id]
        const busy = downloading === item.id
        return (
        <div key={item.id} className="file-entry">
          <div className="file-row">
            <span className="file-icon">{fileIcon(item.name, item.isDir)}</span>
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
              {item.isDir ? null : (
                <>
                  <button
                    className="act-btn"
                    onClick={() => handleDownload(item)}
                    disabled={busy}
                    title="Download (in-app progress)"
                  >
                    {busy ? `${pct ?? 0}%` : '⬇'}
                  </button>
                  <button
                    className="act-btn"
                    onClick={() => handleNativeDownload(item)}
                    title="Download via browser (native progress)"
                  >
                    ↗
                  </button>
                </>
              )}
              <button className="act-btn" onClick={() => onRename(item)} title="Rename" disabled={readOnly} style={readOnly ? { display: 'none' } : undefined}>✏</button>
              <button className="act-btn del" onClick={() => onDelete(item)} title="Delete" disabled={readOnly} style={readOnly ? { display: 'none' } : undefined}>🗑</button>
            </span>
          </div>
          {pct !== undefined && (
            <div className="dl-progress-bar-bg">
              <div className="dl-progress-bar-fill" style={{ width: `${pct}%` }} />
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
