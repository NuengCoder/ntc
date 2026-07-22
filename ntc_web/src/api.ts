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

export function uploadFile(path: string, file: File, onProgress?: (pct: number) => void): Promise<Response> {
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.open('POST', API + '/api/upload')
    xhr.upload.onprogress = (e) => {
      if (e.lengthComputable && onProgress) {
        onProgress(Math.round((e.loaded / e.total) * 100))
      }
    }
    xhr.onload = () => resolve(new Response(xhr.responseText, { status: xhr.status, statusText: xhr.statusText }))
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
