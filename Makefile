env: .env

thread:
	cargo watch -w src -x "lrun --bin thread"
green:
	cargo watch -w src -x "lrun --bin green"