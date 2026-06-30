import { useState, useEffect, useCallback } from 'react'
import type { FileItem } from './types'
import { listFiles, deleteItem as apiDelete } from './api'
import { Breadcrumb } from './components/Breadcrumb'
import { FileList } from './components/FileList'
import { UploadArea } from './components/UploadArea'
import { RenameDialog } from './components/RenameDialog'
import { NewFolderDialog } from './components/NewFolderDialog'
import { ConfirmDialog } from './components/ConfirmDialog'

export default function App() {
  const [path, setPath] = useState('/')
  const [items, setItems] = useState<FileItem[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [renaming, setRenaming] = useState<FileItem | null>(null)
  const [showNewFolder, setShowNewFolder] = useState(false)
  const [deleting, setDeleting] = useState<FileItem | null>(null)
  const [refreshKey, setRefreshKey] = useState(0)

  const load = useCallback(async () => {
    try {
      setLoading(true)
      setError('')
      const res = await listFiles(path)
      setItems(res.items)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setLoading(false)
    }
  }, [path])

  useEffect(() => { load() }, [load, refreshKey])

  function refresh() { setRefreshKey(k => k + 1) }

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
      />

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
          title="Delete"
          body={`Delete "${deleting.name}"?`}
          onConfirm={handleDelete}
          onCancel={() => setDeleting(null)}
        />
      )}
    </div>
  )
}
