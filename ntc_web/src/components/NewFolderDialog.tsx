import { useState } from 'react'
import { createFolder } from '../api'

interface Props {
  currentPath: string
  onDone: () => void
  onClose: () => void
}

export function NewFolderDialog({ currentPath, onDone, onClose }: Props) {
  const [name, setName] = useState('')
  const [err, setErr] = useState('')

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    if (!name.trim()) return
    try {
      await createFolder(name.trim(), currentPath)
      onDone()
    } catch (e: any) {
      setErr(e.message)
    }
  }

  return (
    <div className="overlay" onClick={onClose}>
      <div className="dialog" onClick={e => e.stopPropagation()}>
        <form onSubmit={handleSubmit}>
          <h3>New Folder</h3>
          <input value={name} onChange={e => setName(e.target.value)} placeholder="folder name" autoFocus />
          {err && <p className="err">{err}</p>}
          <div className="dialog-actions">
            <button type="button" onClick={onClose}>Cancel</button>
            <button type="submit">Create</button>
          </div>
        </form>
      </div>
    </div>
  )
}
