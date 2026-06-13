# ntc PowerShell completions
# Install: add to your $PROFILE and run: Register-ArgumentCompleter -CommandName ntc -ScriptBlock $global:ntcCompletion

$global:ntcCompletion = {
    param($wordToComplete, $commandAst, $cursorPosition)

    $opts = @(
        '-i', '--input', '-o', '--output', '-f', '--format', '--cp',
        '--setO', '--setD', '--setL', '--setC', '--setT', '--setH',
        '--showcg', '--watch', '--where', '--size', '--view',
        '--clear', '--version', '--list', '--fun', '--help',
        '--math', '--dino', '--edit', '--init',
        '--ignored', '--ignore', '--cared', '--ignoref', '--caref', '--ignoren', '--caren',
        '--tp-add', '--tp-list', '--tp-rm',
        '--ral-export-all', '--ral-export-select', '--ral-import',
        '--igcare-export-all', '--igcare-export-select', '--igcare-import',
        '-s', '--say', '-say', '-print',
        '--generate-completions'
    )

    $formatOpts = @('txt', 'html', 'json', 'md', 'pdf', 'docx', 'xlsx')
    $boolOpts = @('ON', 'OFF')
    $shellOpts = @('bash', 'zsh', 'fish', 'powershell')

    $prev = $commandAst.CommandElements | Select-Object -Last 1
    $prevText = $prev.Value

    switch ($prevText) {
        '-f' { return $formatOpts }
        '--format' { return $formatOpts }
        '--setL' { return $boolOpts }
        '--setC' { return $boolOpts }
        '--setH' { return @('ON', 'OFF', 'default') }
        '--watch' { return $boolOpts }
        '--generate-completions' { return $shellOpts }
        default {
            if ($wordToComplete -like '-*') {
                return $opts | Where-Object { $_ -like "$wordToComplete*" }
            }
        }
    }
}
