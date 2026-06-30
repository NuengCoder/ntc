interface Props {
  path: string
  onNavigate: (p: string) => void
}

export function Breadcrumb({ path, onNavigate }: Props) {
  const parts = path === '/' ? [] : path.split('/').filter(Boolean)
  const crumbs = [{ label: '~', path: '/' }]
  let acc = ''
  for (const p of parts) {
    acc += '/' + p
    crumbs.push({ label: p, path: acc })
  }
  return (
    <nav className="crumb">
      {crumbs.map((c, i) => (
        <span key={c.path}>
          {i > 0 && <span className="sep">/</span>}
          <button className="crumb-btn" onClick={() => onNavigate(c.path)}>{c.label}</button>
        </span>
      ))}
    </nav>
  )
}
