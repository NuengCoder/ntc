import { useState } from 'react'
import type { FileItem } from '../types'
import { downloadFile } from '../api'

interface Props {
  items: FileItem[]
  loading: boolean
  path: string
  onNavigate: (p: string) => void
  onRename: (item: FileItem) => void
  onDelete: (item: FileItem) => void
  onRefresh: () => void
  onError: (msg: string) => void
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

export function FileList({ items, loading, path, onNavigate, onRename, onDelete, onError }: Props) {
  const dirs = items.filter(i => i.isDir)
  const files = items.filter(i => !i.isDir)
  const [downloading, setDownloading] = useState<number | null>(null)

  // Downloads go through fetch (not a plain <a href>) so the login
  // header is attached when `serve --auth` is on.
  async function handleDownload(item: FileItem) {
    setDownloading(item.id)
    try {
      await downloadFile(item.id, item.name)
    } catch (e: any) {
      onError(e.message || 'Download failed')
    } finally {
      setDownloading(null)
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
      {[...dirs, ...files].map(item => (
        <div key={item.id} className="file-row">
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
              <button
                className="act-btn"
                onClick={() => handleDownload(item)}
                disabled={downloading === item.id}
                title="Download"
              >
                {downloading === item.id ? '…' : '⬇'}
              </button>
            )}
            <button className="act-btn" onClick={() => onRename(item)} title="Rename">✏</button>
            <button className="act-btn del" onClick={() => onDelete(item)} title="Delete">🗑</button>
          </span>
        </div>
      ))}
    </div>
  )
}

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1)
  return (bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1) + ' ' + units[i]
}
