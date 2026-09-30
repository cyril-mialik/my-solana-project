install:
    cargo fetch

validator:
    solana-test-validator

build:
    anchor build

deploy:
    anchor deploy

test:
    cargo test
