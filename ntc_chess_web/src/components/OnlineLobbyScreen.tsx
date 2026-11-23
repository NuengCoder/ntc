import { useEffect, useRef, useState } from 'react'
import {
  ApiError,
  createOnlineRoom,
  getOnlineInfo,
  getOnlineMe,
  getOnlineRoom,
  getSettings,
  joinOnlineRoom,
  listOnlineRooms,
  onlineLogin,
  onlineLogout,
  quickJoinOnline,
  serverHost,
} from '../api'
import { TC_OPTIONS } from '../constants'
import type {
  OnlineInfo,
  OnlineMe,
  OnlineRoomDetails,
  OnlineRoomSummary,
} from '../types'
import BackButton from './BackButton'

interface Props {
  onBack(): void
  onSettings(): void
  onEnterRoom(id: string): void
}

type PendingAction = () => void | Promise<void>

function errorText(e: unknown, fallback: string): string {
  return e instanceof Error ? e.message : fallback
}

export default function OnlineLobbyScreen({ onBack, onSettings, onEnterRoom }: Props) {
  const [info, setInfo] = useState<OnlineInfo | null>(null)
  const [infoError, setInfoError] = useState<string | null>(null)
  const [me, setMe] = useState<OnlineMe | null>(null)
  const [rooms, setRooms] = useState<OnlineRoomSummary[]>([])
  const [roomsLoading, setRoomsLoading] = useState(true)
  const [roomsError, setRoomsError] = useState<string | null>(null)
  const [search, setSearch] = useState(() => {
    try {
      return sessionStorage.getItem('ntc.online.search') ?? ''
    } catch {
      return ''
    }
  })
  const [query, setQuery] = useState(search)
  const [refreshTick, setRefreshTick] = useState(0)

  const [menuOpen, setMenuOpen] = useState(false)
  const [loginOpen, setLoginOpen] = useState(false)
  const [loginName, setLoginName] = useState('')
  const [loginError, setLoginError] = useState<string | null>(null)
  const [loginBusy, setLoginBusy] = useState(false)
  const pendingAction = useRef<PendingAction | null>(null)
  const guestNameRef = useRef<string | null>(null)

  const [selected, setSelected] = useState<OnlineRoomSummary | null>(null)
  const [details, setDetails] = useState<OnlineRoomDetails | null>(null)
  const [detailsLoading, setDetailsLoading] = useState(false)
  const [password, setPassword] = useState('')
  const [joinError, setJoinError] = useState<string | null>(null)
  const [joinBusy, setJoinBusy] = useState(false)

  const [createOpen, setCreateOpen] = useState(false)
  const [createName, setCreateName] = useState('')
  const [createPassword, setCreatePassword] = useState('')
  const [createTc, setCreateTc] = useState('10+0')
  const [createSide, setCreateSide] = useState<'white' | 'black' | 'random'>('random')
  const [createCoach, setCreateCoach] = useState(false)
  const [createError, setCreateError] = useState<string | null>(null)
  const [createBusy, setCreateBusy] = useState(false)
  const [defaultTc, setDefaultTc] = useState('10+0')

  const [quickPending, setQuickPending] = useState(false)
  const [quickError, setQuickError] = useState<string | null>(null)
  const [rulesOpen, setRulesOpen] = useState(false)

  useEffect(() => {
    let live = true
    getOnlineInfo()
      .then((v) => {
        if (live) setInfo(v)
      })
      .catch((e) => {
        if (live) setInfoError(errorText(e, 'Online mode is not available'))
      })
    getOnlineMe()
      .then((v) => {
        if (live) {
          setMe(v)
          guestNameRef.current = v.guest?.name ?? null
        }
      })
      .catch(() => {
        /* no session yet */
      })
    getSettings()
      .then((s) => {
        if (live) setDefaultTc(s['chess.tc'] ?? '10+0')
      })
      .catch(() => {})
    return () => {
      live = false
    }
  }, [])

  useEffect(() => {
    try {
      sessionStorage.setItem('ntc.online.search', search)
    } catch {
      /* private mode */
    }
    const id = window.setTimeout(() => setQuery(search.trim()), 300)
    return () => window.clearTimeout(id)
  }, [search])

  useEffect(() => {
    if (!info?.online) return
    let live = true
    let first = true
    const load = async () => {
      if (first) setRoomsLoading(true)
      try {
        const r = await listOnlineRooms(query)
        if (!live) return
        setRooms(r.rooms)
        setRoomsError(null)
      } catch (e) {
        if (!live) return
        setRoomsError(errorText(e, 'Could not load rooms'))
      } finally {
        if (live && first) setRoomsLoading(false)
        first = false
      }
    }
    void load()
    const id = window.setInterval(() => void load(), 2000)
    return () => {
      live = false
      window.clearInterval(id)
    }
  }, [info?.online, query, refreshTick])

  function requireGuest(action: PendingAction) {
    if (me?.guest) {
      void action()
      return
    }
    pendingAction.current = action
    setLoginName('')
    setLoginError(null)
    setLoginOpen(true)
  }

  async function submitLogin(e: React.FormEvent) {
    e.preventDefault()
    if (loginBusy) return
    setLoginBusy(true)
    setLoginError(null)
    try {
      const r = await onlineLogin(loginName)
      guestNameRef.current = r.guest.name
      setMe((prev) => ({ guest: r.guest, room: prev?.room ?? null }))
      setLoginOpen(false)
      setMenuOpen(false)
      const next = pendingAction.current
      pendingAction.current = null
      if (next) await next()
    } catch (e) {
      setLoginError(errorText(e, 'Could not set your name'))
    } finally {
      setLoginBusy(false)
    }
  }

  async function logout() {
    setMenuOpen(false)
    try {
      await onlineLogout()
    } catch {
      /* still clear the local state */
    }
    guestNameRef.current = null
    setMe({ guest: null, room: null })
    setSelected(null)
    setDetails(null)
  }

  function openLogin() {
    setMenuOpen(false)
    setLoginName(me?.guest?.name ?? '')
    setLoginError(null)
    pendingAction.current = null
    setLoginOpen(true)
  }

  async function openJoin(room: OnlineRoomSummary) {
    if (room.players >= 2 || room.status === 'finished') return
    requireGuest(async () => {
      setSelected(room)
      setDetails(null)
      setDetailsLoading(true)
      setPassword('')
      setJoinError(null)
      try {
        const d = await getOnlineRoom(room.id)
        setDetails(d)
      } catch (e) {
        setJoinError(errorText(e, 'That room is no longer available'))
        setSelected(null)
        setRefreshTick((n) => n + 1)
      } finally {
        setDetailsLoading(false)
      }
    })
  }

  async function submitJoin(e: React.FormEvent) {
    e.preventDefault()
    if (!selected || joinBusy) return
    setJoinBusy(true)
    setJoinError(null)
    try {
      const r = await joinOnlineRoom(selected.id, password)
      setSelected(null)
      setDetails(null)
      onEnterRoom(r.room.id)
    } catch (e) {
      setJoinError(errorText(e, 'Could not join that room'))
      setRefreshTick((n) => n + 1)
    } finally {
      setJoinBusy(false)
    }
  }

  function openCreate() {
    requireGuest(() => {
      setCreateName(`${guestNameRef.current ?? 'Guest'}'s room`.slice(0, 40))
      setCreatePassword('')
      setCreateTc(defaultTc)
      setCreateSide('random')
      setCreateCoach(false)
      setCreateError(null)
      setCreateOpen(true)
    })
  }

  async function submitCreate(e: React.FormEvent) {
    e.preventDefault()
    if (createBusy) return
    setCreateBusy(true)
    setCreateError(null)
    try {
      const r = await createOnlineRoom({
        name: createName,
        password: createPassword.trim() || undefined,
        tc: createTc,
        side: createSide,
        coach_allowed: createCoach,
      })
      setCreateOpen(false)
      onEnterRoom(r.room.id)
    } catch (e) {
      setCreateError(errorText(e, 'Could not create that room'))
    } finally {
      setCreateBusy(false)
    }
  }

  function quickMatch() {
    requireGuest(async () => {
      if (quickPending) return
      setQuickPending(true)
      setQuickError(null)
      try {
        const r = await quickJoinOnline()
        onEnterRoom(r.room.id)
      } catch (e) {
        setQuickError(errorText(e, 'Quick match failed'))
        if (e instanceof ApiError && e.code === 'room_not_found') {
          setRefreshTick((n) => n + 1)
        }
      } finally {
        setQuickPending(false)
      }
    })
  }

  const status = infoError ? 'offline' : info ? 'online' : 'connecting'

  return (
    <div className="screen online-screen">
      <header className="online-topbar">
        <BackButton onBack={onBack} />
        <div className="online-user-wrap">
          <button
            className="online-user"
            onClick={() => (me?.guest ? setMenuOpen((v) => !v) : openLogin())}
          >
            {me?.guest ? me.guest.name : 'Log in'}
          </button>
          {menuOpen && me?.guest && (
            <div className="online-user-menu">
              <button onClick={openLogin}>Change name</button>
              <button onClick={() => void logout()}>Log out</button>
            </div>
          )}
        </div>
        <button onClick={onSettings}>Settings</button>
        <input
          className="online-search"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search rooms"
          maxLength={40}
          aria-label="Search rooms"
        />
        <button onClick={openCreate}>Create</button>
        <button onClick={quickMatch} disabled={quickPending}>
          {quickPending ? 'Joining…' : 'Auto join'}
        </button>
        <button onClick={() => setRulesOpen(true)}>Rules</button>
      </header>

      <main className="online-main">
        {infoError ? (
          <div className="online-disabled">
            <strong>Online play is not enabled on this server.</strong>
            <span>
              Restart it with <code>ntc serve chess --online</code>.
            </span>
            <div className="btn-row">
              <button className="primary" onClick={() => window.location.reload()}>
                Retry
              </button>
              <button onClick={onBack}>Back</button>
            </div>
          </div>
        ) : (
          <section className="online-panel" aria-label="Rooms">
            <div className="online-panel-head">
              <div>
                <strong>Rooms</strong>
                {!roomsLoading && <span className="online-count">{rooms.length}</span>}
              </div>
              <button
                className="online-refresh"
                onClick={() => setRefreshTick((n) => n + 1)}
                title="Refresh rooms"
                aria-label="Refresh rooms"
              >
                ↻
              </button>
            </div>
            {me?.room && (
              <button className="online-resume" onClick={() => onEnterRoom(me.room!.id)}>
                Resume “{me.room.name}”
              </button>
            )}
            {quickError && <div className="online-inline-error">{quickError}</div>}
            {roomsLoading ? (
              <div className="online-skeleton" aria-label="Loading rooms">
                <span />
                <span />
                <span />
              </div>
            ) : roomsError ? (
              <div className="online-empty">
                <span>{roomsError}</span>
                <button onClick={() => setRefreshTick((n) => n + 1)}>Retry</button>
              </div>
            ) : rooms.length === 0 ? (
              <div className="online-empty">
                <span>No rooms yet. Create one.</span>
                <button className="primary" onClick={openCreate}>
                  Create room
                </button>
              </div>
            ) : (
              <div className="online-room-list">
                {rooms.map((room) => {
                  const full = room.players >= 2 || room.status === 'finished'
                  return (
                    <button
                      key={room.id}
                      className={`online-room-row${full ? ' is-full' : ''}`}
                      onClick={() => void openJoin(room)}
                      disabled={full}
                      title={full ? 'Room is full' : `Join ${room.name}`}
                    >
                      <span className="online-room-name">{room.name}</span>
                      {room.locked && (
                        <span className="online-room-lock" aria-label="Password required">
                          [lock]
                        </span>
                      )}
                    </button>
                  )
                })}
              </div>
            )}
          </section>
        )}
      </main>

      <footer className="online-footer">
        <span className={`online-server ${status}`}>
          <i className="online-dot" />
          Server: {serverHost()}
        </span>
        <span className="online-brand">
          <img src="/pieces/wR.png" alt="" className="online-logo" draggable={false} />
          <span>NTC Online Chess</span>
        </span>
      </footer>

      {loginOpen && (
        <div className="modal-back">
          <form className="modal online-dialog" onSubmit={(e) => void submitLogin(e)}>
            <div className="modal-title">Guest name</div>
            <p className="online-dialog-note">
              No account needed. Your name is unique on this server.
            </p>
            <input
              autoFocus
              value={loginName}
              onChange={(e) => setLoginName(e.target.value)}
              maxLength={20}
              placeholder="Your name"
            />
            {loginError && <div className="online-inline-error">{loginError}</div>}
            <div className="btn-row">
              <button className="primary" type="submit" disabled={loginBusy}>
                {loginBusy ? 'Saving…' : 'Continue'}
              </button>
              <button type="button" onClick={() => setLoginOpen(false)}>
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      {selected && (
        <div className="modal-back">
          <form className="modal online-dialog" onSubmit={(e) => void submitJoin(e)}>
            <div className="modal-title">{selected.name}</div>
            {detailsLoading ? (
              <div className="online-dialog-note">Loading room…</div>
            ) : (
              details && (
                <div className="online-details">
                  <div>
                    <span>Host</span>
                    <b>{details.host}</b>
                  </div>
                  <div>
                    <span>Time</span>
                    <b>{details.tc}</b>
                  </div>
                  <div>
                    <span>Open side</span>
                    <b>{details.available_side ?? 'full'}</b>
                  </div>
                  <div>
                    <span>Coach</span>
                    <b>{details.coach_allowed ? '3 per player' : 'off'}</b>
                  </div>
                </div>
              )
            )}
            {details?.locked && (
              <input
                autoFocus
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                maxLength={64}
                placeholder="Room password"
              />
            )}
            {joinError && <div className="online-inline-error">{joinError}</div>}
            <div className="btn-row">
              <button
                className="primary"
                type="submit"
                disabled={joinBusy || detailsLoading || !details || details.full}
              >
                {joinBusy ? 'Joining…' : 'Join'}
              </button>
              <button
                type="button"
                onClick={() => {
                  setSelected(null)
                  setDetails(null)
                }}
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      {createOpen && (
        <div className="modal-back">
          <form className="modal online-dialog" onSubmit={(e) => void submitCreate(e)}>
            <div className="modal-title">Create room</div>
            <label className="online-field">
              <span>Name</span>
              <input
                autoFocus
                value={createName}
                onChange={(e) => setCreateName(e.target.value)}
                maxLength={40}
                required
              />
            </label>
            <label className="online-field">
              <span>Password (optional)</span>
              <input
                type="password"
                value={createPassword}
                onChange={(e) => setCreatePassword(e.target.value)}
                maxLength={64}
                minLength={4}
              />
            </label>
            <label className="online-field">
              <span>Time control</span>
              <select value={createTc} onChange={(e) => setCreateTc(e.target.value)}>
                {TC_OPTIONS.map((o) => (
                  <option key={o} value={o}>
                    {o}
                  </option>
                ))}
              </select>
            </label>
            <label className="online-field">
              <span>Side</span>
              <select
                value={createSide}
                onChange={(e) =>
                  setCreateSide(e.target.value as 'white' | 'black' | 'random')
                }
              >
                <option value="random">Random</option>
                <option value="white">White</option>
                <option value="black">Black</option>
              </select>
            </label>
            <label className="online-check">
              <input
                type="checkbox"
                checked={createCoach}
                onChange={(e) => setCreateCoach(e.target.checked)}
              />
              Allow limited coach (3 uses per player)
            </label>
            {createError && <div className="online-inline-error">{createError}</div>}
            <div className="btn-row">
              <button className="primary" type="submit" disabled={createBusy}>
                {createBusy ? 'Creating…' : 'Create'}
              </button>
              <button type="button" onClick={() => setCreateOpen(false)}>
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      {rulesOpen && (
        <div className="modal-back">
          <div className="modal online-dialog online-rules">
            <div className="modal-title">Online rules & fair play</div>
            <p>Two players, one game. The server is authoritative for moves, clocks, and results.</p>
            <p>
              External engine assistance is prohibited unless the room explicitly allows
              limited coach. Coach-enabled rooms are for casual or learning games and allow
              3 calls per player. The host cannot bypass that limit.
            </p>
            <p>No chat, spectators, ratings, undo, or takeback in this version.</p>
            <p>Password rooms need HTTPS on the public internet. Use a trusted LAN otherwise.</p>
            <div className="btn-row">
              <button className="primary" onClick={() => setRulesOpen(false)}>
                Close
              </button>
            </div>
          </div>
        </div>
      )}

    </div>
  )
}
