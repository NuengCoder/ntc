import { useState, useRef, useCallback } from 'react'
import { uploadFile } from '../api'

interface Props {
  currentPath: string
  onUploaded: () => void
}

interface FileProgress {
  name: string
  pct: number
  done: boolean
  failed: boolean
}

export function UploadArea({ currentPath, onUploaded }: Props) {
  const [dragging, setDragging] = useState(false)
  const [uploading, setUploading] = useState(false)
  const [progList, setProgList] = useState<FileProgress[]>([])
  const inputRef = useRef<HTMLInputElement>(null)

  const handleFiles = useCallback(async (files: FileList) => {
    const arr = Array.from(files)
    setUploading(true)
    setProgList(arr.map(f => ({ name: f.name, pct: 0, done: false, failed: false })))

    const total = arr.length
    let ok = 0
    let fail = 0

    for (let i = 0; i < total; i++) {
      const file = arr[i]
      try {
        await uploadFile(currentPath, file, (pct) => {
          setProgList(prev => {
            const next = [...prev]
            next[i] = { ...next[i], pct }
            return next
          })
        })
        ok++
        setProgList(prev => {
          const next = [...prev]
          next[i] = { ...next[i], pct: 100, done: true }
          return next
        })
      } catch {
        fail++
        setProgList(prev => {
          const next = [...prev]
          next[i] = { ...next[i], failed: true, done: true }
          return next
        })
      }
    }

    setUploading(false)
    if (fail > 0) {
      setProgList([{ name: `Uploaded ${ok}/${total} (${fail} failed)`, pct: 100, done: true, failed: false }])
    } else {
      setProgList([{ name: `Uploaded ${ok} file${ok === 1 ? '' : 's'}`, pct: 100, done: true, failed: false }])
    }
    setTimeout(() => setProgList([]), 3000)
    onUploaded()
  }, [currentPath, onUploaded])

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
      <div className="upload-progress-list">
        {progList.map((p, i) => (
          <div key={i} className="upload-progress-item">
            <div className="upload-progress-info">
              <span className={`upload-progress-name ${p.failed ? 'failed' : ''}`}>
                {p.name}
              </span>
              {p.done ? null : <span className="upload-progress-pct">{p.pct}%</span>}
              {p.failed && <span className="upload-progress-pct failed">FAILED</span>}
            </div>
            {!p.done && (
              <div className="upload-progress-bar-bg">
                <div className="upload-progress-bar-fill" style={{ width: `${p.pct}%` }} />
              </div>
            )}
          </div>
        ))}
      </div>
      {!uploading && progList.length === 0 && (
        <>
          <button className="upload-btn" onClick={() => inputRef.current?.click()}>
            + Upload Files
          </button>
          <span className="upload-hint">or drop files here</span>
        </>
      )}
    </div>
  )
}