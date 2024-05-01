#!/usr/bin/env bats
load bats-extra

# local version: 1.7.0.1

@test 'basic' {
  run bash acronym.bash 'Portable Network Graphics'
  assert_success
  assert_output 'PNG'
}

@test 'lowercase words' {
  run bash acronym.bash 'Ruby on Rails'
  assert_success
  assert_output 'ROR'
}

@test 'punctuation' {
  run bash acronym.bash 'First In, First Out'
  assert_success
  assert_output 'FIFO'
}

@test 'all caps word' {
  run bash acronym.bash 'GNU Image Manipulation Program'
  assert_success
  assert_output 'GIMP'
}

@test 'punctuation without whitespace' {
  run bash acronym.bash 'Complementary metal-oxide semiconductor'
  assert_success
  assert_output 'CMOS'
}

@test 'very long abbreviation' {
  run bash acronym.bash 'Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me'
  assert_success
  assert_output 'ROTFLSHTMDCOALM'
}

@test "consecutive delimiters" {
  run bash acronym.bash "Something - I made up from thin air"
  assert_success
  assert_output "SIMUFTA"
}

@test "apostrophes" {
  run bash acronym.bash "Halley's Comet"
  assert_success
  assert_output "HC"
}

@test "underscore emphasis" {
  run bash acronym.bash "The Road __Not__ Taken"
  assert_success
  assert_output "TRNT"
}

# bash-specific test: Focus the student's attention on the effects of 
# word splitting and filename expansion:
# https://www.gnu.org/software/bash/manual/bash.html#Shell-Expansions

@test "contains shell globbing character" {
  run bash acronym.bash "Two * Words"
  assert_success
  assert_output "TW"
}
