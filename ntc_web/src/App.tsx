import { useState, useEffect, useCallback } from 'react'
import type { FileItem } from './types'
import { listFiles, deleteItem as apiDelete, clearCredentials, hasCredentials } from './api'
import { Breadcrumb } from './components/Breadcrumb'
import { FileList } from './components/FileList'
import { UploadArea } from './components/UploadArea'
import { RenameDialog } from './components/RenameDialog'
import { NewFolderDialog } from './components/NewFolderDialog'
import { ConfirmDialog } from './components/ConfirmDialog'
import { LoginDialog } from './components/LoginDialog'

// The share root is always `/root`: `/` and `/root/` both mean the top folder,
// and every directory below it is `/root/<dir>`, so a copied URL keeps working.
const ROOT_PREFIX = '/root'

function pathFromLocation(): string {
  let raw = window.location.pathname
  try {
    raw = decodeURIComponent(raw)
  } catch {
    // Malformed escape: fall back to the raw path instead of throwing.
  }
  if (raw === ROOT_PREFIX || raw === `${ROOT_PREFIX}/`) return '/'
  if (raw.startsWith(`${ROOT_PREFIX}/`)) {
    const rest = raw.slice(ROOT_PREFIX.length)
    return rest || '/'
  }
  return '/'
}

function urlForPath(path: string): string {
  return path === '/' ? `${ROOT_PREFIX}/` : `${ROOT_PREFIX}${path}`
}

export default function App() {
  const [path, setPath] = useState(pathFromLocation)
  const [items, setItems] = useState<FileItem[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [renaming, setRenaming] = useState<FileItem | null>(null)
  const [showNewFolder, setShowNewFolder] = useState(false)
  const [deleting, setDeleting] = useState<FileItem | null>(null)
  const [refreshKey, setRefreshKey] = useState(0)
  // True when the server answered 401 (`serve --auth`): block the UI
  // behind the login dialog until valid credentials are stored.
  const [locked, setLocked] = useState(false)
  const [authed, setAuthed] = useState(() => hasCredentials())
  // True when the server runs `serve files --lock` (read-only).
  const [readOnly, setReadOnly] = useState(false)

  // Navigation keeps the address bar in sync (`/root/<dir>`), so reloading or
  // copying the URL lands on the same directory.
  const navigate = useCallback((next: string) => {
    setPath(next)
    const url = urlForPath(next)
    if (window.location.pathname !== url) {
      window.history.pushState(null, '', url)
    }
  }, [])

  useEffect(() => {
    const onPop = () => setPath(pathFromLocation())
    window.addEventListener('popstate', onPop)
    return () => window.removeEventListener('popstate', onPop)
  }, [])

  const load = useCallback(async () => {
    try {
      setLoading(true)
      setError('')
      const res = await listFiles(path)
      setItems(res.items)
      setReadOnly(!!res.locked)
    } catch (e: any) {
      if (e.status === 401) {
        setLocked(true)
      } else {
        setError(e.message)
      }
    } finally {
      setLoading(false)
    }
  }, [path])

  useEffect(() => { load() }, [load, refreshKey])

  // Any API call (upload/rename/delete/download) can surface a 401 when
  // the session is missing or stale — `api.ts` broadcasts it globally.
  useEffect(() => {
    const onAuthFail = () => setLocked(true)
    window.addEventListener('ntc:unauthorized', onAuthFail)
    return () => window.removeEventListener('ntc:unauthorized', onAuthFail)
  }, [])

  function refresh() { setRefreshKey(k => k + 1) }

  function handleUnlock() {
    setLocked(false)
    setAuthed(hasCredentials())
    refresh()
  }

  function handleLogout() {
    clearCredentials()
    setAuthed(false)
    setItems([])
    setLocked(true)
  }

  const handleDelete = async () => {
    if (!deleting) return
    const fullPath = path === '/' ? `/${deleting.name}` : `${path}/${deleting.name}`
    try {
      await apiDelete(fullPath)
    } catch (e: any) {
      setError(e.message)
    }
    setDeleting(null)
    refresh()
  }

  return (
    <div className="app">
      <header className="top-bar">
        <h1>ntc serve{readOnly ? ' (read-only)' : ''}</h1>
        <div className="top-actions">
          {authed && <button className="top-btn" onClick={handleLogout} title="Log out">Logout</button>}
          {!readOnly && <button className="top-btn" onClick={() => setShowNewFolder(true)}>+ Folder</button>}
          <button className="top-btn" onClick={refresh} title="Refresh">Refresh</button>
          {/* /info is served by Rust: it explains the endpoints and recipes. */}
          <a className="top-btn" href="/info" target="_blank" rel="noopener noreferrer" title="How to upload/download from a terminal">Info</a>
        </div>
      </header>

      {readOnly && <div className="error-bar">Read-only mode: downloads allowed, uploads/edits disabled.</div>}

      <Breadcrumb path={path} onNavigate={navigate} />

      {error && <div className="error-bar">{error}</div>}

      {!readOnly && <UploadArea currentPath={path} onUploaded={refresh} />}

      <FileList
        items={items}
        loading={loading}
        path={path}
        onNavigate={navigate}
        onRename={setRenaming}
        onDelete={setDeleting}
        onRefresh={refresh}
        onError={setError}
        readOnly={readOnly}
      />

      {locked && <LoginDialog onDone={handleUnlock} />}

      {renaming && (
        <RenameDialog
          currentPath={path}
          oldName={renaming.name}
          onDone={() => { setRenaming(null); refresh() }}
          onClose={() => setRenaming(null)}
        />
      )}

      {showNewFolder && (
        <NewFolderDialog
          currentPath={path}
          onDone={() => { setShowNewFolder(false); refresh() }}
          onClose={() => setShowNewFolder(false)}
        />
      )}

      {deleting && (
        <ConfirmDialog
          title={deleting.isDir ? 'Delete folder' : 'Delete'}
          body={deleting.isDir ? `Delete folder "${deleting.name}" and all its contents?` : `Delete "${deleting.name}"?`}
          onConfirm={handleDelete}
          onCancel={() => setDeleting(null)}
        />
      )}
    </div>
  )
}
