# Bash completion for breath

_breath() {
    local cur prev opts subcommands
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # Subcommands
    subcommands="config health commit push pull status log diff"

    # Global options
    opts="-h --help -V --version"

    # If we're at position 1, suggest subcommands
    if [ ${COMP_CWORD} -eq 1 ]; then
        COMPREPLY=( $(compgen -W "${subcommands} ${opts}" -- ${cur}) )
        return 0
    fi

    # Get the subcommand
    local subcommand="${COMP_WORDS[1]}"

    # Handle subcommand-specific completions
    case "${subcommand}" in
        config)
            if [ ${COMP_CWORD} -eq 2 ]; then
                COMPREPLY=( $(compgen -W "git hg" -- ${cur}) )
            else
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            fi
            ;;
        health|commit|push|pull|status|log|diff)
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            ;;
        *)
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            ;;
    esac

    return 0
}

complete -F _breath breath