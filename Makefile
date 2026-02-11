clean:
	cargo clean -v

build:
	cargo build --profile release

run:
	cargo run --profile release

list_mouse:
	ls -l /dev/input/by-id/ | grep "event-mouse"

list_keyboard:
	ls -l /dev/input/by-id/ | grep "event-kbd"