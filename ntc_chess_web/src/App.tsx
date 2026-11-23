import { useState } from 'react'
import AiVsAiScreen from './components/AiVsAiScreen'
import BotScreen from './components/BotScreen'
import GameScreen from './components/GameScreen'
import HelpScreen from './components/HelpScreen'
import OnlineGameScreen from './components/OnlineGameScreen'
import OnlineLobbyScreen from './components/OnlineLobbyScreen'
import SettingsScreen from './components/SettingsScreen'
import TutorialScreen from './components/TutorialScreen'

type Screen =
  | 'menu'
  | 'game'
  | 'ai'
  | 'online'
  | 'online-game'
  | 'bot'
  | 'settings'
  | 'help'
  | 'tutorial'

const BACK_RANK = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']

function heroPiece(row: number, col: number): string | null {
  if (row === 0) return `b${BACK_RANK[col]}`
  if (row === 1) return 'bP'
  if (row === 6) return 'wP'
  if (row === 7) return `w${BACK_RANK[col]}`
  return null
}

function HeroBoard() {
  return (
    <div className="hero-board" aria-hidden="true">
      {Array.from({ length: 64 }, (_, i) => {
        const row = Math.floor(i / 8)
        const col = i % 8
        const code = heroPiece(row, col)
        return (
          <div
            key={i}
            className={`hero-square ${(row + col) % 2 === 0 ? 'light' : 'dark'}`}
          >
            {code && <img src={`/pieces/${code}.png`} alt="" draggable={false} />}
          </div>
        )
      })}
    </div>
  )
}

export default function App() {
  const [screen, setScreen] = useState<Screen>('menu')
  const [roomId, setRoomId] = useState<string | null>(null)
  const [settingsReturn, setSettingsReturn] = useState<Screen>('menu')

  function openSettings(from: Screen) {
    setSettingsReturn(from)
    setScreen('settings')
  }

  if (screen === 'game') return <GameScreen onExit={() => setScreen('menu')} />
  if (screen === 'ai')
    return <AiVsAiScreen onBack={() => setScreen('menu')} onStart={() => setScreen('game')} />
  if (screen === 'online')
    return (
      <OnlineLobbyScreen
        onBack={() => setScreen('menu')}
        onSettings={() => openSettings('online')}
        onEnterRoom={(id) => {
          setRoomId(id)
          setScreen('online-game')
        }}
      />
    )
  if (screen === 'online-game' && roomId)
    return (
      <OnlineGameScreen
        roomId={roomId}
        onBack={() => {
          setRoomId(null)
          setScreen('online')
        }}
      />
    )
  if (screen === 'bot')
    return <BotScreen onBack={() => setScreen('menu')} onStart={() => setScreen('game')} />
  if (screen === 'settings')
    return <SettingsScreen onBack={() => setScreen(settingsReturn)} />
  if (screen === 'help') return <HelpScreen onBack={() => setScreen('menu')} />
  if (screen === 'tutorial') return <TutorialScreen onBack={() => setScreen('menu')} />

  return (
    <div className="app-shell">
      <header className="site-header">
        <button className="brand" onClick={() => setScreen('menu')} aria-label="ntc chess home">
          <img src="/pieces/wK.png" alt="" draggable={false} />
          <span>
            ntc<strong>chess</strong>
          </span>
        </button>
        <nav className="site-nav" aria-label="Main">
          <button className="nav-item active">Play</button>
          <button className="nav-item" onClick={() => setScreen('bot')}>
            Bots
          </button>
          <button className="nav-item" onClick={() => setScreen('online')}>
            Online
          </button>
          <button className="nav-item" onClick={() => setScreen('help')}>
            Learn
          </button>
        </nav>
        <div className="header-actions">
          <button className="icon-button" onClick={() => openSettings('menu')} title="Settings">
            <svg viewBox="0 0 24 24" width="19" height="19" aria-hidden="true">
              <path
                d="M12 8.4a3.6 3.6 0 1 0 0 7.2 3.6 3.6 0 0 0 0-7.2Zm9 4.6-2.1-1.6.4-2.6-2.5-1.4-2.2 1.3-2.2-1.3-2.5 1.4.4 2.6L1 12l2.1 1.6-.4 2.6 2.5 1.4 2.2-1.3 2.2 1.3 2.5-1.4-.4-2.6L21 13Z"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.6"
                strokeLinejoin="round"
              />
            </svg>
          </button>
          <span className="user-chip">
            <span className="user-avatar">G</span>
            Guest
          </span>
        </div>
      </header>

      <main className="home-main">
        <section className="home-hero">
          <div className="hero-copy">
            <span className="eyebrow">
              <span className="live-dot" /> Play chess
            </span>
            <h1>
              Your next move.
              <br />
              <em>Right here.</em>
            </h1>
            <p>
              Play locally with a friend, challenge a local engine, or open a room and
              share the board. No account needed.
            </p>
            <div className="hero-actions">
              <button className="primary big" onClick={() => setScreen('game')}>
                Play now
              </button>
              <button className="big ghost" onClick={() => setScreen('bot')}>
                Challenge a bot
              </button>
            </div>
            <div className="hero-stats">
              <span>
                <strong>4</strong> game modes
              </span>
              <span>
                <strong>100%</strong> local engines
              </span>
              <span>
                <strong>∞</strong> rematches
              </span>
            </div>
          </div>
          <div className="hero-art">
            <div className="hero-glow" />
            <HeroBoard />
            <div className="floating-card">
              <img src="/pieces/wN.png" alt="" draggable={false} />
              <span>
                <strong>Ready when you are</strong>
                <small>Click Play now to take the board</small>
              </span>
            </div>
          </div>
        </section>

        <section className="section-head">
          <h2>Choose a game</h2>
          <span>Pick a mode and start playing</span>
        </section>
        <section className="mode-grid">
          <button className="mode-card featured" onClick={() => setScreen('game')}>
            <span className="mode-icon">
              <img src="/pieces/wK.png" alt="" draggable={false} />
            </span>
            <span className="mode-copy">
              <strong>Play</strong>
              <small>Player vs Player · local hotseat</small>
            </span>
            <span className="mode-arrow">→</span>
          </button>
          <button className="mode-card" onClick={() => setScreen('bot')}>
            <span className="mode-icon">
              <img src="/pieces/bK.png" alt="" draggable={false} />
            </span>
            <span className="mode-copy">
              <strong>Play bots</strong>
              <small>Stockfish · four strength levels</small>
            </span>
            <span className="mode-arrow">→</span>
          </button>
          <button className="mode-card" onClick={() => setScreen('ai')}>
            <span className="mode-icon">
              <img src="/pieces/bN.png" alt="" draggable={false} />
            </span>
            <span className="mode-copy">
              <strong>AI vs AI</strong>
              <small>Watch two engines battle it out</small>
            </span>
            <span className="mode-arrow">→</span>
          </button>
          <button className="mode-card" onClick={() => setScreen('online')}>
            <span className="mode-icon">
              <img src="/pieces/wQ.png" alt="" draggable={false} />
            </span>
            <span className="mode-copy">
              <strong>Play online</strong>
              <small>Create or join a room with a friend</small>
            </span>
            <span className="mode-arrow">→</span>
          </button>
        </section>

        <section className="quick-grid">
          <button className="quick-card" onClick={() => setScreen('tutorial')}>
            <span className="quick-kicker">New to chess?</span>
            <strong>Learn the basics</strong>
            <span className="quick-link">Open tutorial →</span>
          </button>
          <button className="quick-card" onClick={() => openSettings('menu')}>
            <span className="quick-kicker">Make it yours</span>
            <strong>Board, sound &amp; coach</strong>
            <span className="quick-link">Open settings →</span>
          </button>
          <button className="quick-card" onClick={() => setScreen('help')}>
            <span className="quick-kicker">Need a hand?</span>
            <strong>Help &amp; notation</strong>
            <span className="quick-link">Read the guide →</span>
          </button>
        </section>
      </main>
    </div>
  )
}
