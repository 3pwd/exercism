#!/usr/bin/env bats
load bats-extra

# local version: 1.1.0.0

@test "the sound for 1 is 1" {
  #[[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 1
  assert_success
  assert_output "1"
}

@test "the sound for 3 is Pling" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 3
  assert_success
  assert_output "Pling"
}

@test "the sound for 5 is Plang" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 5
  assert_success
  assert_output "Plang"
}

@test "the sound for 7 is Plong" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 7
  assert_success
  assert_output "Plong"
}

@test "the sound for 6 is Pling as it has a factor 3" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 6
  assert_success
  assert_output "Pling"
}

@test "2 to the power 3 does not make a raindrop sound as 3 is the exponent not the base" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 8
  assert_success
  assert_output "8"
}

@test "the sound for 9 is Pling as it has a factor 3" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 9
  assert_success
  assert_output "Pling"
}

@test "the sound for 10 is Plang as it has a factor 5" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 10
  assert_success
  assert_output "Plang"
}

@test "the sound for 14 is Plong as it has a factor of 7" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 14
  assert_success
  assert_output "Plong"
}

@test "the sound for 15 is PlingPlang as it has factors 3 and 5" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 15
  assert_success
  assert_output "PlingPlang"
}

@test "the sound for 21 is PlingPlong as it has factors 3 and 7" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 21
  assert_success
  assert_output "PlingPlong"
}

@test "the sound for 25 is Plang as it has a factor 5" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 25
  assert_success
  assert_output "Plang"
}

@test "the sound for 27 is Pling as it has a factor 3" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 27
  assert_success
  assert_output "Pling"
}

@test "the sound for 35 is PlangPlong as it has factors 5 and 7" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 35
  assert_success
  assert_output "PlangPlong"
}

@test "the sound for 49 is Plong as it has a factor 7" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 49
  assert_success
  assert_output "Plong"
}

@test "the sound for 52 is 52" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 52
  assert_success
  assert_output "52"
}

@test "the sound for 105 is PlingPlangPlong as it has factors 3, 5 and 7" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 105
  assert_success
  assert_output "PlingPlangPlong"
}

@test "the sound for 3125 is Plang as it has a factor 5" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash raindrops.bash 3125
  assert_success
  assert_output "Plang"
}
