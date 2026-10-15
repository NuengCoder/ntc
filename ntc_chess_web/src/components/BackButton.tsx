export default function BackButton({ onBack }: { onBack(): void }) {
  return (
    <button className="back-btn" onClick={onBack} title="Back to menu">
      <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
        <path
          d="M10 3 L5 8 L10 13"
          fill="none"
          stroke="currentColor"
          strokeWidth="2.2"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
      <span>Menu</span>
    </button>
  )
}
