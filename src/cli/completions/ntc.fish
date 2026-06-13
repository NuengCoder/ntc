complete -c ntc -f

# Input/Output
complete -c ntc -s i -l input -d 'Input file or directory path' -r
complete -c ntc -s o -l output -d 'Output filename' -r
complete -c ntc -s f -l format -d 'Output format' -r -f -a 'txt html json md pdf docx xlsx'
complete -c ntc -l cp -d 'Copy report to clipboard'

# Configuration
complete -c ntc -l setO -d 'Show or set output directory' -r
complete -c ntc -l setD -d 'Show or set max depth (1-20)' -r
complete -c ntc -l setL -d 'Show or set line numbers' -r -f -a 'ON OFF'
complete -c ntc -l setC -d 'Show or set color output' -r -f -a 'ON OFF'
complete -c ntc -l setT -d 'Show or set thread count' -r
complete -c ntc -l setH -d 'Show or set history path/state' -r
complete -c ntc -l showcg -d 'Show current configuration'
complete -c ntc -l watch -d 'Show or set file watcher state' -r -f -a 'ON OFF'

# Info
complete -c ntc -l where -d 'Show ntc executable and config location'
complete -c ntc -l version -d 'Show version information'
complete -c ntc -l list -l fun -d 'List all command-line functions'
complete -c ntc -l help -d 'Show detailed help'
complete -c ntc -l clear -d 'Clear the terminal screen'

# Tools
complete -c ntc -l size -d 'Show current directory size'
complete -c ntc -l view -d 'Quick view of current directory tree'
complete -c ntc -l math -d 'Evaluate a math expression' -r
complete -c ntc -l dino -d 'Play the dinosaur runner game'
complete -c ntc -l generate-completions -d 'Generate shell completions' -r -f -a 'bash zsh fish powershell'

# Editor
complete -c ntc -s e -l edit -d 'Open file in built-in text editor' -r
complete -c ntc -l init -d 'Create starter file from template' -r

# Ignore/Care
complete -c ntc -l ignored -d 'Show ignored items'
complete -c ntc -l ignore -d 'Ignore directory names' -r
complete -c ntc -l cared -d 'Stop ignoring directories' -r
complete -c ntc -l ignoref -d 'Ignore file extensions' -r
complete -c ntc -l caref -d 'Care about file extensions' -r
complete -c ntc -l ignoren -d 'Ignore specific files' -r
complete -c ntc -l caren -d 'Care about specific files' -r

# Teleport
complete -c ntc -l tp-add -d 'Save directory as teleport point' -r
complete -c ntc -l tp-list -d 'List all teleport points'
complete -c ntc -l tp-rm -d 'Remove teleport point' -r

# Run Aliases
complete -c ntc -l ral-export-all -d 'Export all run aliases' -r
complete -c ntc -l ral-export-select -d 'Select run aliases to export' -r
complete -c ntc -l ral-import -d 'Import run aliases from file' -r

# Ignore/Care export/import
complete -c ntc -l igcare-export-all -d 'Export all ignore/care settings' -r
complete -c ntc -l igcare-export-select -d 'Select categories to export' -r
complete -c ntc -l igcare-import -d 'Import ignore/care settings' -r

# Say
complete -c ntc -s s -l say -d 'Print text to stdout' -r
