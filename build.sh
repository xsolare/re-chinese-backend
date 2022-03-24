#!/bin/bash

function yellow() { printf "\x1b[38;5;227m %s\e[0m " "${@}"; printf "\n"; }
function warn() { printf "\x1b[38;5;208m %s\e[0m " "${@}"; printf "\n"; }
function green() { printf "\x1b[38;5;048m %s\e[0m " "${@}"; printf "\n"; }
function red() { printf "\x1b[38;5;196m %s\e[0m " "${@}"; printf "\n"; }

function build() {
    yellow "building rust rest api base image"
    time docker build -f ./Dockerfile --rm -t evvvai/re-chinese-backend:latest .
    var_last_status="$?"
    if [[ "${var_last_status}" -ne 0 ]]; then
        red "error - failed to build rust rest api base image - stopping"
        exit 1
    fi
}

build

green "done building docker base image"

exit 0
