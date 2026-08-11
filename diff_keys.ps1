$ErrorActionPreference = 'Stop'
$raw = [System.IO.File]::ReadAllText('src/utils/config/lang/en.rs')
$rx = [regex]'m\.insert\(\s*"([^"]+)"\.into\(\),\s*"((?:[^"\\]|\\.)*)"\.into\(\)'
$ms = $rx.Matches($raw)
$pairs = @()
foreach ($mm in $ms) {
    $pairs += [pscustomobject]@{ Key = $mm.Groups[1].Value; Val = $mm.Groups[2].Value }
}
$unique = @{}
foreach ($p in $pairs) {
    if (-not $unique.ContainsKey($p.Key)) {
        $unique[$p.Key] = $p.Val
    }
}
Write-Output ("PARSED={0} UNIQUE={1}" -f $pairs.Count, $unique.Count)
$covered = @{}
Get-ChildItem src/utils/config/lang/th/part*.rs | ForEach-Object {
    $c = Get-Content $_.FullName
    foreach ($l in $c) {
        if ($l -match '^\s*m\.insert\("(.*?)"\.into\(\)') { $covered[$Matches[1]] = $true }
    }
}
$missing = @($unique.Keys | Where-Object { -not $covered.ContainsKey($_) })
# keep en.rs order
$ordered = @($pairs | Select-Object -ExpandProperty Key -Unique)
$missing = @($ordered | Where-Object { -not $covered.ContainsKey($_) })
Write-Output "MISSING=$($missing.Count)"
$prefixCount = @{}
foreach ($k in $missing) { $p = ($k -split '\.')[0]; if (-not $prefixCount.ContainsKey($p)) { $prefixCount[$p] = 0 }; $prefixCount[$p]++ }
foreach ($p in ($prefixCount.Keys | Sort-Object)) { Write-Output ("  {0} = {1}" -f $p, $prefixCount[$p]) }
Set-Content -Path missing_keys.txt -Value $missing -Encoding UTF8
Set-Content -Path missing_pairs.txt -Value (@($missing | ForEach-Object { $_ + "`t" + $unique[$_] })) -Encoding UTF8