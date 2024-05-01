#!/usr/bin/env bash

main() {
  IFS=""
  local result

  while read -rn 1 letter; do
    result="$letter$result"
  done < <(printf %s "${1}")

  printf "%s\n" "$result"
}

main "$@"
