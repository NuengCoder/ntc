import { useState } from 'react'
import { setCredentials, clearCredentials, listFiles } from '../api'

interface Props {
  onDone: () => void
}

export function LoginDialog({ onDone }: Props) {
  const [user, setUser] = useState('')
  const [pass, setPass] = useState('')
  const [err, setErr] = useState('')
  const [busy, setBusy] = useState(false)

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    if (!user.trim() || busy) return
    setBusy(true)
    setErr('')
    setCredentials(user.trim(), pass)
    try {
      await listFiles('/')
      onDone()
    } catch (e: any) {
      clearCredentials()
      setErr(e.status === 401 ? 'Wrong username or password' : (e.message || 'Login failed'))
      setBusy(false)
    }
  }

  return (
    <div className="overlay">
      <div className="dialog" onClick={e => e.stopPropagation()}>
        <form onSubmit={handleSubmit}>
          <h3>🔒 Login required</h3>
          <input
            value={user}
            onChange={e => setUser(e.target.value)}
            placeholder="Username"
            autoFocus
            autoComplete="username"
            disabled={busy}
          />
          <input
            type="password"
            value={pass}
            onChange={e => setPass(e.target.value)}
            placeholder="Password"
            autoComplete="current-password"
            disabled={busy}
          />
          {err && <p className="err">{err}</p>}
          <div className="dialog-actions">
            <button type="submit" disabled={busy}>{busy ? 'Checking…' : 'Unlock'}</button>
          </div>
        </form>
      </div>
    </div>
  )
}
