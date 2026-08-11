import type { FilesResponse } from './types'

const API = ''

async function api<T>(url: string, init?: RequestInit): Promise<T> {
  const res = await fetch(API + url, init)
  if (!res.ok) {
    const body = await res.text()
    let msg = body
    try { const j = JSON.parse(body); msg = j.error || body } catch {}
    throw new Error(msg || `HTTP ${res.status}`)
  }
  return res.json()
}

export function listFiles(path: string): Promise<FilesResponse> {
  const p = path === '/' ? '/' : path
  return api(`/api/files?path=${encodeURIComponent(p)}`)
}

export function downloadUrl(id: number): string {
  return `/api/download?id=${id}`
}

const CHUNK_SIZE = 80 * 1024 * 1024 // 80 MB, keeps each request under the tunnel's ~100 MB cap

function parseErr(text: string, status: number): string {
  let msg = text.trim()
  try { const j = JSON.parse(text); msg = j.error || msg } catch {}
  return msg || `HTTP ${status}`
}

function xhrUpload(url: string, form: FormData, onUploadProgress?: (loaded: number) => void): Promise<void> {
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.open('POST', API + url)
    xhr.upload.onprogress = (e) => {
      if (e.lengthComputable && onUploadProgress) onUploadProgress(e.loaded)
    }
    xhr.onload = () => {
      if (xhr.status < 200 || xhr.status >= 300) {
        reject(new Error(parseErr(xhr.responseText, xhr.status)))
        return
      }
      resolve()
    }
    xhr.onerror = () => reject(new Error('Network error'))
    xhr.ontimeout = () => reject(new Error('Upload timed out'))
    xhr.send(form)
  })
}

async function uploadChunked(path: string, file: File, onProgress?: (pct: number) => void): Promise<void> {
  const uploadId =
    typeof crypto !== 'undefined' && 'randomUUID' in crypto
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2)}`
  const totalSize = file.size
  const totalChunks = Math.ceil(totalSize / CHUNK_SIZE)

  for (let i = 0; i < totalChunks; i++) {
    const start = i * CHUNK_SIZE
    const end = Math.min(start + CHUNK_SIZE, totalSize)
    const blob = file.slice(start, end)
    const form = new FormData()
    form.append('file', blob, file.name)
    form.append('uploadId', uploadId)
    form.append('chunkIndex', String(i))
    form.append('totalChunks', String(totalChunks))
    form.append('totalSize', String(totalSize))
    form.append('path', path)
    try {
      await xhrUpload('/api/upload-chunk', form, (loaded) => {
        if (onProgress) onProgress(Math.round(((start + loaded) / totalSize) * 100))
      })
    } catch (err) {
      try { await fetch(API + '/api/upload-cancel?uploadId=' + encodeURIComponent(uploadId), { method: 'DELETE' }) } catch {}
      throw err
    }
  }

  const res = await fetch(API + '/api/upload-finalize', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ uploadId, path, filename: file.name, size: totalSize }),
  })
  if (!res.ok) {
    const text = await res.text()
    try { await fetch(API + '/api/upload-cancel?uploadId=' + encodeURIComponent(uploadId), { method: 'DELETE' }) } catch {}
    throw new Error(parseErr(text, res.status))
  }
}

export function uploadFile(path: string, file: File, onProgress?: (pct: number) => void): Promise<Response> {
  if (file.size > CHUNK_SIZE) {
    return new Promise((resolve, reject) => {
      uploadChunked(path, file, onProgress).then(
        () => resolve(new Response('{"uploaded":1}', { status: 200 })),
        reject,
      )
    })
  }
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.open('POST', API + '/api/upload')
    xhr.upload.onprogress = (e) => {
      if (e.lengthComputable && onProgress) {
        onProgress(Math.round((e.loaded / e.total) * 100))
      }
    }
    xhr.onload = () => {
      const res = new Response(xhr.responseText, { status: xhr.status, statusText: xhr.statusText })
      if (!res.ok) {
        reject(new Error(parseErr(xhr.responseText, xhr.status)))
        return
      }
      resolve(res)
    }
    xhr.onerror = () => reject(new Error('Network error'))
    xhr.ontimeout = () => reject(new Error('Upload timed out'))
    const form = new FormData()
    form.append('file', file)
    form.append('path', path)
    xhr.send(form)
  })
}

export function createFolder(name: string, path: string): Promise<FilesResponse> {
  return api('/api/create-folder', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name, path }),
  })
}

export function deleteItem(filePath: string): Promise<FilesResponse> {
  return api(`/api/delete?path=${encodeURIComponent(filePath)}`, { method: 'DELETE' })
}

export function renameItem(oldPath: string, newName: string): Promise<FilesResponse> {
  return api('/api/rename', {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ oldPath, newName }),
  })
}
