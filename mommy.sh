#!/bin/sh
mommyexec() {
    if [ $? = 0 ]; then
        mommy positive
    else
        mommy negative
    fi
}

export PROMPT_COMMAND="mommyexec; $PROMPT_COMMAND"