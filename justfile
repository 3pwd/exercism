fmt:
    cargo fmt --all & taplo fmt rust/
    shfmt -w -i 2 bash/**/*.bash

lint:
    cargo clippy --all
    find bash -type f -name "*.bash" ! -name "bats-extra.bash" -exec shellcheck -a {} +

bash-tests:
    find bash -mindepth 1 -maxdepth 1 -type d -exec bash -c 'cd "{}" && bats -r .' \;

bash-test exercise:
    cd bash/{{exercise}} && bats -r .

rust-tests:
    cargo nextest run --all --cargo-quiet

rust-test exercise:
    cargo nextest run -p {{exercise}} --cargo-quiet
