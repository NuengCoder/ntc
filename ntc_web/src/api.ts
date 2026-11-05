import type { FilesResponse } from './types'

const API = ''

// Stored as base64(user:pass) in sessionStorage (cleared when the tab
// closes). Attached as an HTTP Basic header when `serve --auth` is on;
// when the server has no auth configured the header is simply absent.
const CRED_KEY = 'ntc.serve.auth'

export class ApiError extends Error {
  status: number
  constructor(message: string, status: number) {
    super(message)
    this.name = 'ApiError'
    this.status = status
  }
}

function notifyUnauthorized() {
  window.dispatchEvent(new CustomEvent('ntc:unauthorized'))
}

export function setCredentials(user: string, pass: string) {
  const raw = `${user}:${pass}`
  const encoded = btoa(unescape(encodeURIComponent(raw)))
  sessionStorage.setItem(CRED_KEY, encoded)
}

export function clearCredentials() {
  sessionStorage.removeItem(CRED_KEY)
}

export function hasCredentials(): boolean {
  return sessionStorage.getItem(CRED_KEY) !== null
}

function authHeaders(): Record<string, string> {
  const c = sessionStorage.getItem(CRED_KEY)
  return c ? { Authorization: `Basic ${c}` } : {}
}

async function api<T>(url: string, init?: RequestInit): Promise<T> {
  const res = await fetch(API + url, {
    ...init,
    headers: { ...authHeaders(), ...(init?.headers || {}) },
  })
  if (!res.ok) {
    const body = await res.text()
    let msg = body
    try { const j = JSON.parse(body); msg = j.error || body } catch {}
    if (res.status === 401) notifyUnauthorized()
    throw new ApiError(msg || `HTTP ${res.status}`, res.status)
  }
  return res.json()
}

export function listFiles(path: string): Promise<FilesResponse> {
  const p = path === '/' ? '/' : path
  return api(`/api/files?path=${encodeURIComponent(p)}`)
}

// Stable, shareable link for a file inside the share root. `/download/root`
// mirrors the `/root/...` browse URL: `/root/hello.c` downloads from
// `/download/root/hello.c`.
export function shareUrl(filePath: string): string {
  return API + '/download/root' + encodeURI(filePath)
}

export interface DownloadHandle {
  pause: () => void
  resume: () => void
  cancel: () => void
  done: Promise<void>
}

// In-app download with real pause/resume: the body is read as a stream, so
// pausing keeps the bytes already received and resuming asks the server for
// the rest with `Range: bytes=<received>-`.
export function startDownload(
  filePath: string,
  name: string,
  onProgress: (pct: number) => void,
): DownloadHandle {
  const chunks: Uint8Array[] = []
  let received = 0
  let paused = false
  let cancelled = false
  let controller: AbortController | null = null
  let settled = false
  let resolveDone: () => void = () => {}
  let rejectDone: (e: Error) => void = () => {}
  const done = new Promise<void>((resolve, reject) => {
    resolveDone = resolve
    rejectDone = reject
  })

  function finish(err?: Error) {
    if (settled) return
    settled = true
    if (err) rejectDone(err)
    else resolveDone()
  }

  function save() {
    const blob = new Blob(chunks as BlobPart[])
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = name
    document.body.appendChild(a)
    a.click()
    a.remove()
    setTimeout(() => URL.revokeObjectURL(url), 5000)
  }

  // One request. Resolves true when the whole file has arrived, false when
  // the server sent a shorter range and another request is needed.
  async function fetchOnce(): Promise<boolean> {
    const ctrl = new AbortController()
    controller = ctrl
    const headers: Record<string, string> = { ...authHeaders() }
    if (received > 0) headers['Range'] = `bytes=${received}-`
    const res = await fetch(shareUrl(filePath), { headers, signal: ctrl.signal })
    if (res.status === 401) notifyUnauthorized()
    if (!res.ok && res.status !== 206) {
      throw new ApiError(`HTTP ${res.status}`, res.status)
    }
    const cr = res.headers.get('Content-Range')
    const cl = res.headers.get('Content-Length')
    const total = cr
      ? parseInt((/\/(\d+)\s*$/.exec(cr) || [])[1] || '0', 10)
      : received + (cl ? parseInt(cl, 10) : 0)
    const reader = res.body?.getReader()
    if (!reader) throw new Error('Streaming is not supported by this browser')
    for (;;) {
      const { done: end, value } = await reader.read()
      if (end) break
      if (value) {
        chunks.push(value)
        received += value.byteLength
        if (total > 0) onProgress(Math.min(100, Math.floor((received / total) * 100)))
      }
    }
    return total === 0 || received >= total
  }

  async function pump() {
    while (!paused && !cancelled) {
      try {
        if (await fetchOnce()) {
          onProgress(100)
          save()
          finish()
          return
        }
      } catch (e: any) {
        // An abort is a pause/cancel, not a failure.
        if (paused || cancelled) return
        finish(e instanceof Error ? e : new Error(String(e)))
        return
      }
    }
  }

  void pump()

  return {
    pause: () => {
      if (settled) return
      paused = true
      controller?.abort()
    },
    resume: () => {
      if (settled || !paused) return
      paused = false
      void pump()
    },
    cancel: () => {
      if (settled) return
      cancelled = true
      controller?.abort()
      finish(new Error('cancelled'))
    },
    done,
  }
}

// Plain-link URL for browser-native progress (download tray). Without
// --auth it is the share link itself; with --auth a short-lived id token is
// appended so the link works without the Basic header.
export async function getNativeDownloadUrl(filePath: string, id: number): Promise<string> {
  if (!hasCredentials()) {
    return shareUrl(filePath)
  }
  const res = await api<{ token: string; url: string }>(`/api/download-token`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ id }),
  })
  return `${shareUrl(filePath)}?id=${id}&token=${encodeURIComponent(res.token)}`
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
    const creds = sessionStorage.getItem(CRED_KEY)
    if (creds) xhr.setRequestHeader('Authorization', `Basic ${creds}`)
    xhr.upload.onprogress = (e) => {
      if (e.lengthComputable && onUploadProgress) onUploadProgress(e.loaded)
    }
    xhr.onload = () => {
      if (xhr.status === 401) notifyUnauthorized()
      if (xhr.status < 200 || xhr.status >= 300) {
        reject(new ApiError(parseErr(xhr.responseText, xhr.status), xhr.status))
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
      try { await fetch(API + '/api/upload-cancel?uploadId=' + encodeURIComponent(uploadId), { method: 'DELETE', headers: authHeaders() }) } catch {}
      throw err
    }
  }

  const res = await fetch(API + '/api/upload-finalize', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...authHeaders() },
    body: JSON.stringify({ uploadId, path, filename: file.name, size: totalSize }),
  })
  if (!res.ok) {
    const text = await res.text()
    if (res.status === 401) notifyUnauthorized()
    try { await fetch(API + '/api/upload-cancel?uploadId=' + encodeURIComponent(uploadId), { method: 'DELETE', headers: authHeaders() }) } catch {}
    throw new ApiError(parseErr(text, res.status), res.status)
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
    const creds = sessionStorage.getItem(CRED_KEY)
    if (creds) xhr.setRequestHeader('Authorization', `Basic ${creds}`)
    xhr.upload.onprogress = (e) => {
      if (e.lengthComputable && onProgress) {
        onProgress(Math.round((e.loaded / e.total) * 100))
      }
    }
    xhr.onload = () => {
      const res = new Response(xhr.responseText, { status: xhr.status, statusText: xhr.statusText })
      if (xhr.status === 401) notifyUnauthorized()
      if (!res.ok) {
        reject(new ApiError(parseErr(xhr.responseText, xhr.status), xhr.status))
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
