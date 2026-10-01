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

export default function App() {
  const [path, setPath] = useState('/')
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

  const load = useCallback(async () => {
    try {
      setLoading(true)
      setError('')
      const res = await listFiles(path)
      setItems(res.items)
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
        <h1>ntc serve</h1>
        <div className="top-actions">
          {authed && <button className="top-btn" onClick={handleLogout} title="Log out">🔒 Logout</button>}
          <button className="top-btn" onClick={() => setShowNewFolder(true)}>+ Folder</button>
          <button className="top-btn" onClick={refresh}>↻</button>
        </div>
      </header>

      <Breadcrumb path={path} onNavigate={setPath} />

      {error && <div className="error-bar">{error}</div>}

      <UploadArea currentPath={path} onUploaded={refresh} />

      <FileList
        items={items}
        loading={loading}
        path={path}
        onNavigate={setPath}
        onRename={setRenaming}
        onDelete={setDeleting}
        onRefresh={refresh}
        onError={setError}
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
