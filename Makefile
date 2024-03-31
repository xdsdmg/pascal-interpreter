TEST_CODE = ./test-code/code-1.pas

test-lexer:
	cargo test test_lexer -- --nocapture -- $(TEST_CODE)

test-parser:
	RUST_BACKTRACE=1 cargo test test_parser -- --nocapture -- $(TEST_CODE)

test-interpreter:
	export RUST_BACKTRACE=1; cargo run $(TEST_CODE)
