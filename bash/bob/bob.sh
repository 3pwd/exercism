#!/usr/bin/env bash

set -o errexit
set -o nounset

answer() {
  printf "%s\n" "$1" && exit 0
}

is_question() {
  [[ $sentence =~ \?$ ]]
}

answer_question() {
  is_question && answer "Sure."
}

is_yelling() {
  local upper_cased=${sentence^^}
  [[ -n "$sentence" && $upper_cased == "$sentence" && $sentence =~ [A-Z] ]]
}

answer_yelling() {
  is_yelling && answer "Whoa, chill out!"
}

is_yelling_question() {
  is_question && is_yelling
}

answer_yelling_question() {
  is_yelling_question && answer "Calm down, I know what I'm doing!"
}

is_silence() {
  [[ -z $sentence ]]
}

answer_silence() {
  is_silence && answer "Fine. Be that way!"
}

main() {
  local sentence="${1-}"
  sentence=${sentence//[[:space:]]/}

  answer_silence ||
  answer_yelling_question ||
  answer_yelling ||
  answer_question ||
  answer "Whatever."
}

main "$@"
