import { useState, useRef } from 'react'
import { uploadFile } from '../api'

interface Props {
  currentPath: string
  onUploaded: () => void
}

export function UploadArea({ currentPath, onUploaded }: Props) {
  const [dragging, setDragging] = useState(false)
  const [uploading, setUploading] = useState(false)
  const [progress, setProgress] = useState('')
  const inputRef = useRef<HTMLInputElement>(null)

  async function handleFiles(files: FileList) {
    setUploading(true)
    const total = files.length
    let done = 0
    let failed = 0
    for (const file of files) {
      done++
      setProgress(`${done}/${total}: ${file.name}…`)
      try {
        await uploadFile(currentPath, file)
      } catch (e) {
        failed++
        console.error('Upload failed:', file.name, e)
      }
    }
    setUploading(false)
    setProgress(failed > 0 ? `Uploaded ${done - failed}/${total} (${failed} failed)` : `Uploaded ${done} file${done === 1 ? '' : 's'}`)
    setTimeout(() => setProgress(''), 3000)
    onUploaded()
  }

  return (
    <div
      className={`upload-area ${dragging ? 'drag' : ''}`}
      onDragOver={e => { e.preventDefault(); setDragging(true) }}
      onDragLeave={() => setDragging(false)}
      onDrop={e => { e.preventDefault(); setDragging(false); handleFiles(e.dataTransfer.files) }}
    >
      <input
        ref={inputRef}
        type="file"
        multiple
        hidden
        onChange={e => e.target.files && handleFiles(e.target.files)}
      />
      {uploading ? (
        <span className="upload-status">{progress}</span>
      ) : (
        <>
          <button className="upload-btn" onClick={() => inputRef.current?.click()}>
            + Upload Files
          </button>
          <span className="upload-hint">or drop files here</span>
          {progress && <span className="upload-status" style={{ display: 'block', marginTop: 6 }}>{progress}</span>}
        </>
      )}
    </div>
  )
}
