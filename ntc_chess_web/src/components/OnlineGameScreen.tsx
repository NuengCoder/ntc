import { useCallback, useMemo, useState } from 'react'
import {
  getOnlineMoves,
  getOnlinePosition,
  getOnlineState,
  leaveOnlineRoom,
  onlineCoach,
  onlineDraw,
  onlineMove,
  onlineRematch,
  onlineResign,
} from '../api'
import GameScreen, { type GameCapabilities, type GameTransport } from './GameScreen'

interface Props {
  roomId: string
  onBack(): void
}

const ONLINE_CAPABILITIES: GameCapabilities = {
  canUndo: false,
  canNewGame: false,
  canRematch: true,
  canDraw: true,
  canCoach: true,
  coachIsLimited: true,
}

export default function OnlineGameScreen({ roomId, onBack }: Props) {
  const [gone, setGone] = useState(false)

  const transport = useMemo<GameTransport>(
    () => ({
      getState: (signal) => getOnlineState(roomId, signal),
      getMoves: (sq) => getOnlineMoves(roomId, sq),
      getPosition: (ply) => getOnlinePosition(roomId, ply),
      move: (from, to, promo) => onlineMove(roomId, from, to, promo),
      resign: () => onlineResign(roomId),
      draw: (action) => onlineDraw(roomId, action ?? 'offer'),
      rematch: () => onlineRematch(roomId),
      getCoach: async (signal?: AbortSignal) => (await onlineCoach(roomId, signal)).coach,
    }),
    [roomId],
  )

  const exit = useCallback(async () => {
    try {
      await leaveOnlineRoom(roomId)
    } catch {
      /* the room may already be gone */
    } finally {
      onBack()
    }
  }, [roomId, onBack])

  if (gone) {
    return (
      <div className="screen online-screen">
        <div className="online-gone">
          <strong>This room is no longer available.</strong>
          <span>The host may have restarted the server or the room expired.</span>
          <button className="primary" onClick={onBack}>
            Back to lobby
          </button>
        </div>
      </div>
    )
  }

  return (
    <GameScreen
      onExit={() => void exit()}
      transport={transport}
      capabilities={ONLINE_CAPABILITIES}
      onRoomGone={() => setGone(true)}
    />
  )
}
