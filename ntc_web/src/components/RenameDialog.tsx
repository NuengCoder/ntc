import { useState } from 'react'
import { renameItem } from '../api'

interface Props {
  currentPath: string
  oldName: string
  onDone: () => void
  onClose: () => void
}

export function RenameDialog({ currentPath, oldName, onDone, onClose }: Props) {
  const [name, setName] = useState(oldName)
  const [err, setErr] = useState('')
  const fullPath = currentPath === '/' ? `/${oldName}` : `${currentPath}/${oldName}`

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    if (!name.trim()) return
    try {
      await renameItem(fullPath, name.trim())
      onDone()
    } catch (e: any) {
      setErr(e.message)
    }
  }

  return (
    <div className="overlay" onClick={onClose}>
      <div className="dialog" onClick={e => e.stopPropagation()}>
        <form onSubmit={handleSubmit}>
          <h3>Rename</h3>
          <input value={name} onChange={e => setName(e.target.value)} autoFocus />
          {err && <p className="err">{err}</p>}
          <div className="dialog-actions">
            <button type="button" onClick={onClose}>Cancel</button>
            <button type="submit">Rename</button>
          </div>
        </form>
      </div>
    </div>
  )
}
