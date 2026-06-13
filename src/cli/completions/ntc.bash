_ntc_completions() {
    local cur prev words cword
    _init_completion || return

    local opts="
        -i --input -o --output -f --format --cp
        --setO --setD --setL --setC --setT --setH
        --showcg --watch --where --size --view
        --clear --version --list --fun --help
        --math --dino --edit --init
        --ignored --ignore --cared --ignoref --caref --ignoren --caren
        --tp-add --tp-list --tp-rm
        --ral-export-all --ral-export-select --ral-import
        --igcare-export-all --igcare-export-select --igcare-import
        --say -say -print --generate-completions
    "

    local formats="txt html json md pdf docx xlsx"
    local bools="ON OFF"
    local shells="bash zsh fish powershell"

    case $prev in
        -i|--input)
            _filedir
            return
            ;;
        -o|--output)
            _filedir
            return
            ;;
        -f|--format)
            COMPREPLY=($(compgen -W "$formats" -- "$cur"))
            return
            ;;
        --setL|--setC|--setH|--watch)
            COMPREPLY=($(compgen -W "$bools" -- "$cur"))
            return
            ;;
        --setO|--setD|--setT)
            return
            ;;
        --generate-completions)
            COMPREPLY=($(compgen -W "$shells" -- "$cur"))
            return
            ;;
        --math)
            return
            ;;
        --edit|--init)
            _filedir
            return
            ;;
        --ignore|--cared|--ignoref|--caref|--ignoren|--caren)
            return
            ;;
        --tp-add|--tp-rm|--ral-export-all|--ral-export-select|--ral-import|--igcare-export-all|--igcare-export-select|--igcare-import)
            return
            ;;
    esac

    if [[ "$cur" == -* ]]; then
        COMPREPLY=($(compgen -W "$opts" -- "$cur"))
    fi
}

complete -F _ntc_completions ntc
