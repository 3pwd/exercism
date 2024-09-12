alias db := download-bash
alias dr := download-rust
alias f := fmt
alias fcb := _fmt-check-bash
alias fcr := _fmt-check-rust
alias l := lint
alias lb := lint-bash
alias lr := lint-rust
alias sr := submit-rust
alias t := tests
alias tb := test-bash
alias tab := tests-bash
alias tr := test-rust
alias tar := tests-rust


default:
    @just --list

# Format (write) all files
fmt:
    @cargo fmt --all & taplo fmt rust/
    @find bash -type f -name "*.bash" ! -name "bats-extra.bash" -exec shfmt -w -i 2 {} +

_fmt-check-rust:
    @cargo fmt --all -- --check
    @taplo fmt rust/ --check

_fmt-check-bash:
    @find bash -type f -name "*.bash"! -name "bats-extra.bash" -exec shfmt -d -i 2 {} +

# Lint all files
lint: lint-rust lint-bash

# Lint Rust files
lint-rust:
    @cargo clippy --all

# Lint Bash files
lint-bash:
    @find bash -type f -name "*.bash" ! -name "bats-extra.bash" -exec shellcheck -a {} +

submit-rust exercise:
    @exercism submit $(find rust/{{exercise}}/src -name '*.rs') rust/{{exercise}}/Cargo.toml

# Run all tests
tests: tests-rust tests-bash

# Run all Bats tests
tests-bash:
    @find bash -mindepth 1 -maxdepth 1 -type d -exec bash -c 'cd "{}" && bats -r .' \;

# Run Bats tests for a specific exercise
test-bash exercise:
    @cd bash/{{exercise}} && bats -r .

# Run all Rust tests
tests-rust:
    @cargo nextest run --all --cargo-quiet

_tests-rust-ci:
    @cargo test --all

# Run Rust tests for a specific exercise
test-rust exercise:
    @cargo nextest run -p {{exercise}} --cargo-quiet

# Download an exercise
_download exercise track:
    @exercism download --exercise={{exercise}} --track={{track}}

# Download a bash exercise
download-bash exercise:
    @just _download {{exercise}} bash

_add-rust exercise:
    #! /bin/bash
    main() {
      local new_member="$1"
      local cargo_toml="Cargo.toml"

      dasel put -r toml -f "$cargo_toml" -v "rust/$new_member" 'workspace.members.append()'
      local members=$(dasel -r toml -f "$cargo_toml" -w json 'workspace.members' | jq -c 'sort')
      dasel put -r toml -f "$cargo_toml" -w toml -t json -v "$members" workspace.members
      taplo fmt "$cargo_toml"
    }

    main {{exercise}}

# Download a rust exercise
download-rust exercise:
    @just _download {{exercise}} rust
    @just _add-rust {{exercise}}
