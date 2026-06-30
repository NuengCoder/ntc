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

export function uploadFile(path: string, file: File): Promise<Response> {
  const form = new FormData()
  form.append('file', file)
  form.append('path', path)
  return fetch(API + '/api/upload', { method: 'POST', body: form })
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
