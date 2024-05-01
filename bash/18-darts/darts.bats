#!/usr/bin/env bats
load bats-extra

# local version: 2.2.0.1

@test "Missed target" {
    run bash darts.bash -9 9
    assert_success
    assert_output "0"
}

@test "On the outer circle" {
    run bash darts.bash 0 10
    assert_success
    assert_output "1"
}

@test "On the middle circle" {
    run bash darts.bash -5 0
    assert_success
    assert_output "5"
}

@test "On the inner circle" {
    run bash darts.bash 0 -1
    assert_success
    assert_output "10"
}

@test "Exactly on centre" {
    run bash darts.bash 0 0
    assert_success
    assert_output "10"
}

@test "Near the centre" {
    run bash darts.bash -0.1 -0.1
    assert_success
    assert_output "10"
}

@test "Just within the inner circle" {
    run bash darts.bash 0.7 0.7
    assert_success
    assert_output "10"
}

@test "Just outside the inner circle" {
    run bash darts.bash 0.8 -0.8
    assert_success
    assert_output "5"
}

@test "Just within the middle circle" {
    run bash darts.bash -3.5 3.5
    assert_success
    assert_output "5"
}   

@test "Just outside the middle circle" {
    run bash darts.bash -3.6 -3.6
    assert_success
    assert_output "1"
}     

@test "Just within the outer circle" {
    run bash darts.bash -7.0 7.0
    assert_success
    assert_output "1"
}     

@test "Just outside the outer circle" {
    run bash darts.bash 7.1 -7.1
    assert_success
    assert_output "0"
}    

@test "Asymmetric position between the inner and middle circles" {
    run bash darts.bash 0.5 -4
    assert_success
    assert_output "5"
}

# bash-specific test: Input validation

@test "invalid args: no args" {
    run bash darts.bash
    assert_failure
    assert_output    # there is _some_ output
}

@test "invalid args: only 1 arg" {
    run bash darts.bash 10
    assert_failure
    assert_output    # there is _some_ output
}

@test "invalid args: first arg non-numeric" {
    run bash darts.bash foo 10
    assert_failure
    assert_output    # there is _some_ output
}

@test "invalid args: second arg non-numeric" {
    run bash darts.bash 10 bar
    assert_failure
    assert_output    # there is _some_ output
}
