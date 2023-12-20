.PHONY: run trace-basic_opration

run:
	cargo run

trace-basic_opration:
	RUST_LOG=trace cargo run --example basic_opration

trace-form_json_rejection:
	RUST_LOG=trace cargo run --example form_json_rejection

trace-askama_template:
	RUST_LOG=trace cargo run --example askama_template

trace-database:
	RUST_LOG=trace cargo run --example database

watch:  
	RUST_LOG=debug cargo watch -q -c -w examples/ -x "run --example todolist"

watch-test:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

clean:
	cargo clean