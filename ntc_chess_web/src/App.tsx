import { useState } from 'react'
import BotScreen from './components/BotScreen'
import GameScreen from './components/GameScreen'
import HelpScreen from './components/HelpScreen'
import SettingsScreen from './components/SettingsScreen'
import TutorialScreen from './components/TutorialScreen'

type Screen = 'menu' | 'game' | 'bot' | 'settings' | 'help' | 'tutorial'

export default function App() {
  const [screen, setScreen] = useState<Screen>('menu')

  if (screen === 'game') return <GameScreen onExit={() => setScreen('menu')} />
  if (screen === 'bot')
    return <BotScreen onBack={() => setScreen('menu')} onStart={() => setScreen('game')} />
  if (screen === 'settings') return <SettingsScreen onBack={() => setScreen('menu')} />
  if (screen === 'help') return <HelpScreen onBack={() => setScreen('menu')} />
  if (screen === 'tutorial') return <TutorialScreen onBack={() => setScreen('menu')} />

  return (
    <div className="screen menu">
      <img src="/pieces/wK.png" className="menu-logo" alt="ntc chess logo" draggable={false} />
      <h1>ntc chess</h1>
      <p className="subtitle">2-player hotseat or train against a local engine</p>
      <div className="menu-list">
        <button className="primary big" onClick={() => setScreen('game')}>
          2 Player
        </button>
        <button onClick={() => setScreen('bot')}>vs AI Bot</button>
        <button disabled title="not available yet">
          Online (soon)
        </button>
        <button onClick={() => setScreen('settings')}>Settings</button>
        <button onClick={() => setScreen('help')}>Help</button>
        <button onClick={() => setScreen('tutorial')}>Tutorial</button>
      </div>
    </div>
  )
}
