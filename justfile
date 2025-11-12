set shell := ["nu", "-c"]

debug:
    @cargo run

run:
    @cargo run --release
