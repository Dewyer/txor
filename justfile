
run-cli-test:
    cargo run --release -- ./test_data/txs.csv > ./test_data/accounts.csv

run-test:
    cargo test -- --nocapture