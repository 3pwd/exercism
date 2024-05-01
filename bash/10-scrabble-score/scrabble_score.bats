#!/usr/bin/env bats
load bats-extra

# local version: 1.1.0.0

@test 'lowercase letter' {
  #[[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'a'
  
  assert_success
  assert_output "1"
}

@test 'uppercase letter' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'A'
  
  assert_success
  assert_output "1"
}

@test 'valuable letter' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'f'
  
  assert_success
  assert_output "4"
}

@test 'short word' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'at'
  
  assert_success
  assert_output "2"
}

@test 'short, valuable word' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'zoo'
  
  assert_success
  assert_output "12"
}

@test 'medium word' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'street'
  
  assert_success
  assert_output "6"
}

@test 'medium, valuable word' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'quirky'
  
  assert_success
  assert_output "22"
}

@test 'long, mixed-case word' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'OxyphenButazone'
  
  assert_success
  assert_output "41"
}

@test 'english-like word' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'pinata'
  
  assert_success
  assert_output "8"
}

@test 'empty input' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash ''
  
  assert_success
  assert_output "0"
}

@test 'entire alphabet available' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash scrabble_score.bash 'abcdefghijklmnopqrstuvwxyz'
  
  assert_success
  assert_output "87"
}
