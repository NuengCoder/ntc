$ErrorActionPreference = 'Stop'
$lines = Get-Content missing_pairs.txt
$prev = $null
$start = 1
for ($i = 0; $i -lt $lines.Count; $i++) {
    $k = ($lines[$i] -split "`t")[0]
    $pfx = ($k -split '\.')[0]
    if ($pfx -ne $prev) {
        if ($prev -ne $null) { Write-Output ("{0}: lines {1}-{2}" -f $prev, $start, $i) }
        $prev = $pfx
        $start = $i + 1
    }
}
Write-Output ("{0}: lines {1}-{2}" -f $prev, $start, $lines.Count)