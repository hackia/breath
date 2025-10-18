# Fish completion for breath

# Main command completions
complete -c breath -f

# Subcommands
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "config" -d "Manage configuration"
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "health" -d "Verify repository health"
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "commit" -d "Commit changes to the repository"
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "push" -d "Push changes to remote repositories"
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "pull" -d "Pull changes from remote repositories"
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "status" -d "Show the status of the repository"
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "log" -d "Show the commit log"
complete -c breath -n "not __fish_seen_subcommand_from config health commit push pull status log diff" -a "diff" -d "Show the changes between the working directory and the index"

# Global options
complete -c breath -s h -l help -d "Print help information"
complete -c breath -s V -l version -d "Print version information"

# Config subcommand options
complete -c breath -n "__fish_seen_subcommand_from config" -a "git" -d "Configure Git repository"
complete -c breath -n "__fish_seen_subcommand_from config" -a "hg" -d "Configure Mercurial repository"