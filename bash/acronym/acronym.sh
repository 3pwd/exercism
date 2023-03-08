#!/usr/bin/env bash

main () {
  IFS=" -_*"
  local -a sentence
  read -ra sentence <<< "${1^^}"

  for word in "${sentence[@]}";do
    printf "%c" "$word"
  done

  printf "\\n"
}

main "$@"
