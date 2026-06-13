#compdef ntc

_ntc_completions() {
    local -a opts
    opts=(
        '(-i --input)'{-i,--input}'[Input file or directory path]:file:_files'
        '(-o --output)'{-o,--output}'[Output filename]:file:_files'
        '(-f --format)'{-f,--format}'[Output format]:format:(txt html json md pdf docx xlsx)'
        '--cp[Copy report to clipboard]'
        '--setO[Show or set output directory]:path:_files'
        '--setD[Show or set max depth (1-20)]:depth:'
        '--setL[Show or set line numbers]:state:(ON OFF)'
        '--setC[Show or set color output]:state:(ON OFF)'
        '--setT[Show or set thread count]:threads:'
        '--setH[Show or set history path/state]:value:'
        '--showcg[Show current configuration]'
        '--watch[Show or set file watcher state]:state:(ON OFF)'
        '--where[Show ntc executable and config location]'
        '--size[Show current directory size]'
        '--view[Quick view of current directory tree]'
        '--clear[Clear the terminal screen]'
        '--version[Show version information]'
        '--list[--fun][List all command-line functions]'
        '--help[Show detailed help]'
        '--math[Evaluate a math expression]:expression:'
        '--dino[Play the dinosaur runner game]'
        '--edit[Open file in built-in text editor]:file:_files'
        '--init[Create starter file from template]:file:_files'
        '--ignored[Show ignored items]'
        '--ignore[Ignore directory names]:name:'
        '--cared[Stop ignoring directories]:name:'
        '--ignoref[Ignore file extensions]:ext:'
        '--caref[Care about file extensions]:ext:'
        '--ignoren[Ignore specific files]:file:'
        '--caren[Care about specific files]:file:'
        '--tp-add[Save directory as teleport point]:name:'
        '--tp-list[List all teleport points]'
        '--tp-rm[Remove teleport point]:name:'
        '--ral-export-all[Export all run aliases]:name:'
        '--ral-export-select[Select run aliases to export]:name:'
        '--ral-import[Import run aliases from file]:file:_files'
        '--igcare-export-all[Export all ignore/care settings]:name:'
        '--igcare-export-select[Select categories to export]:name:'
        '--igcare-import[Import ignore/care settings]:file:_files'
        '--generate-completions[Generate shell completions]:shell:(bash zsh fish powershell)'
        '(-s --say)'{-s,--say}'[Print text to stdout]:text:'
    )

    _arguments -C "$opts[@]"
}

_ntc_completions "$@"
