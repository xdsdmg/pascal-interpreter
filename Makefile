test-lexer:
	cargo test test_lexer -- --nocapture

test-parser:
	RUST_BACKTRACE=1 cargo test test_parser -- --nocapture

test:
	export RUST_BACKTRACE=1; cargo run ./code.pas
