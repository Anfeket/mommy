#!/bin/zsh
function mommyexec() {
    if [[ $? = 0 ]]; then
        mommy positive
    else
        mommy negative
    fi
}

typeset -a precmd_functions
precmd_functions+=(mommyexec)