.PHONY: run trace watch watch-test clean

run:
	cargo run

trace:
	RUST_LOG=trace cargo run

watch:
	cargo watch -q -c -w src/ -x run

watch-test:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

clean:
	cargo clean