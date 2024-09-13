#!/usr/bin/env bash

main() {
  if [[ $# = 1 ]]; then
    echo "Hello, $1"
    return 0
  else
    echo "Usage: error_handling.sh <person>"
    return 1
  fi
}

main "$@"
