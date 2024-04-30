alias f := fmt
alias fcb := fmt-check-bash
alias fcr := fmt-check-rust
alias lb := lint-bash
alias l := lint
alias lr := lint-rust
alias t := tests
alias tb := test-bash
alias tab := tests-bash
alias tr := test-rust
alias tar := tests-rust

fmt:
    cargo fmt --all & taplo fmt rust/
    shfmt -w -i 2 bash/**/*.bash

fmt-check-rust:
    cargo fmt --all -- --check
    taplo fmt rust/ --check

fmt-check-bash:
    shfmt -d bash/**/*.bash

lint: lint-rust lint-bash

lint-rust:
    cargo clippy --all

lint-bash:
    find bash -type f -name "*.bash" ! -name "bats-extra.bash" -exec shellcheck -a {} +

tests: tests-rust tests-bash

tests-bash:
    find bash -mindepth 1 -maxdepth 1 -type d -exec bash -c 'cd "{}" && bats -r .' \;

test-bash exercise:
    cd bash/{{exercise}} && bats -r .

tests-rust:
    cargo nextest run --all --cargo-quiet

_tests-rust-ci:
    cargo test --all

test-rust exercise:
    cargo nextest run -p {{exercise}} --cargo-quiet
