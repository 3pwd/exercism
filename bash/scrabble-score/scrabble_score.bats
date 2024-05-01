#!/usr/bin/env bats
load bats-extra

# local version: 1.1.0.0

@test 'lowercase letter' {
  run bash scrabble_score.bash 'a'
  
  assert_success
  assert_output "1"
}

@test 'uppercase letter' {
  run bash scrabble_score.bash 'A'
  
  assert_success
  assert_output "1"
}

@test 'valuable letter' {
  run bash scrabble_score.bash 'f'
  
  assert_success
  assert_output "4"
}

@test 'short word' {
  run bash scrabble_score.bash 'at'
  
  assert_success
  assert_output "2"
}

@test 'short, valuable word' {
  run bash scrabble_score.bash 'zoo'
  
  assert_success
  assert_output "12"
}

@test 'medium word' {
  run bash scrabble_score.bash 'street'
  
  assert_success
  assert_output "6"
}

@test 'medium, valuable word' {
  run bash scrabble_score.bash 'quirky'
  
  assert_success
  assert_output "22"
}

@test 'long, mixed-case word' {
  run bash scrabble_score.bash 'OxyphenButazone'
  
  assert_success
  assert_output "41"
}

@test 'english-like word' {
  run bash scrabble_score.bash 'pinata'
  
  assert_success
  assert_output "8"
}

@test 'empty input' {
  run bash scrabble_score.bash ''
  
  assert_success
  assert_output "0"
}

@test 'entire alphabet available' {
  run bash scrabble_score.bash 'abcdefghijklmnopqrstuvwxyz'
  
  assert_success
  assert_output "87"
}
